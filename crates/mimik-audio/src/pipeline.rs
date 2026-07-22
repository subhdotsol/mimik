use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use mimik_core::voice::{load_dsp_preset, load_manifest, Effect};
use mimik_dsp::effects::{
    Chorus, DeEsser, HighPassFilter, LimiterDb, NoiseGateDb, PeakingEq, PitchShifter,
    Reverb, SampleProcessor, SmoothCompressor, Vibrato,
};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};
use std::{fs, path::Path, time::Duration};

pub const PID_FILE: &str = "/tmp/mimik.pid";

const VOICES_DIR: &str = "assets/voices";

fn build_sample_chains(
    effects: &[Effect],
    sample_rate: f32,
) -> (
    Vec<Box<dyn SampleProcessor>>,
    f32,
    f64,
    Vec<Box<dyn SampleProcessor>>,
) {
    let mut pre: Vec<Box<dyn SampleProcessor>> = Vec::new();
    let mut post: Vec<Box<dyn SampleProcessor>> = Vec::new();
    let mut pitch_semitones = 0.0f32;
    let mut formant_ratio = 1.0f64;
    let mut past_pitch = false;

    for effect in effects {
        match effect {
            Effect::PitchShift { semitones } => {
                pitch_semitones = *semitones;
                past_pitch = true;
            }
            Effect::FormantShift { ratio } => {
                formant_ratio = *ratio as f64;
            }
            Effect::NoiseGate { threshold_db } => {
                let p: Box<dyn SampleProcessor> = Box::new(NoiseGateDb::new(*threshold_db));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::HighPass { frequency_hz } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(HighPassFilter::new(*frequency_hz, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Equalizer { frequency_hz, gain_db, q } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(PeakingEq::new(*frequency_hz, *gain_db, *q, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Compressor { threshold_db, ratio, attack_ms, release_ms } => {
                let p: Box<dyn SampleProcessor> = Box::new(SmoothCompressor::new(
                    *threshold_db, *ratio, *attack_ms, *release_ms, sample_rate,
                ));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Limiter { ceiling_db } => {
                let p: Box<dyn SampleProcessor> = Box::new(LimiterDb::new(*ceiling_db));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Vibrato { rate_hz, depth_cents, mix } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(Vibrato::new(*rate_hz, *depth_cents, *mix, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::DeEsser { frequency_hz, threshold_db } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(DeEsser::new(*frequency_hz, *threshold_db, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Chorus { rate_hz, depth_ms, mix } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(Chorus::new(*rate_hz, *depth_ms, *mix, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
            Effect::Reverb { room_size, mix, decay_ms } => {
                let p: Box<dyn SampleProcessor> =
                    Box::new(Reverb::new(*room_size, *mix, *decay_ms, sample_rate));
                if past_pitch { post.push(p) } else { pre.push(p) }
            }
        }
    }

    (pre, pitch_semitones, formant_ratio, post)
}

pub fn run(voice: Option<&str>) -> Result<()> {
    let voice_name = voice.unwrap_or("clean");

    let preset = if voice_name == "clean" {
        None
    } else {
        let voice_dir = Path::new(VOICES_DIR).join(voice_name);
        let manifest = load_manifest(&voice_dir)
            .with_context(|| {
                format!("Voice '{voice_name}' not found. Run 'mimik list' to see installed voices.")
            })?;

        if manifest.engine != "dsp" {
            anyhow::bail!(
                "Voice '{voice_name}' uses engine '{}' which is not yet supported",
                manifest.engine
            );
        }

        Some(load_dsp_preset(&voice_dir)?)
    };

    let host = cpal::default_host();

    let input_device = host
        .default_input_device()
        .context("No default input microphone found")?;

    let output_device = host
        .output_devices()?
        .find(|device| {
            device
                .description()
                .map(|d| d.name().contains("BlackHole 2ch"))
                .unwrap_or(false)
        })
        .context("BlackHole 2ch not found. Install from existential.audio/blackhole")?;

    let input_config = input_device.default_input_config()?;
    let output_config = output_device.default_output_config()?;

    let input_channels = input_config.channels() as usize;
    let output_channels = output_config.channels() as usize;
    let sample_rate = input_config.sample_rate();

    println!(
        "Input:  {} ch @ {} Hz  {:?}",
        input_channels, sample_rate, input_config.sample_format()
    );
    println!(
        "Output: {} ch @ {} Hz  {:?}",
        output_channels, output_config.sample_rate(), output_config.sample_format()
    );

    if input_config.sample_format() != cpal::SampleFormat::F32 {
        anyhow::bail!("Input format {:?} not yet supported", input_config.sample_format());
    }
    if output_config.sample_format() != cpal::SampleFormat::F32 {
        anyhow::bail!("Output format {:?} not yet supported", output_config.sample_format());
    }
    if sample_rate != output_config.sample_rate() {
        anyhow::bail!(
            "Sample rate mismatch: input {} Hz vs output {} Hz",
            sample_rate, output_config.sample_rate()
        );
    }

    match preset {
        None => {
            let input_sc: cpal::StreamConfig = input_config.into();
            let output_sc: cpal::StreamConfig = output_config.into();

            let ring = HeapRb::<f32>::new(sample_rate as usize * 2);
            let (mut producer, mut consumer) = ring.split();

            let _input_stream = input_device.build_input_stream(
                input_sc,
                move |input: &[f32], _| {
                    for frame in input.chunks(input_channels) {
                        let mono = frame.iter().sum::<f32>() / frame.len() as f32;
                        let _ = producer.try_push(mono);
                    }
                },
                |e| eprintln!("Input error: {e}"),
                None,
            )?;

            let _output_stream = output_device.build_output_stream(
                output_sc,
                move |output: &mut [f32], _| {
                    for frame in output.chunks_mut(output_channels) {
                        let s = consumer.try_pop().unwrap_or(0.0);
                        for ch in frame { *ch = s; }
                    }
                },
                |e| eprintln!("Output error: {e}"),
                None,
            )?;

            _input_stream.play()?;
            _output_stream.play()?;

            write_pid(voice_name)?;
            println!("Mimik is live [clean]");
            println!("Speak into your microphone. Ctrl+C to stop.");
            loop { std::thread::sleep(Duration::from_secs(1)); }
        }

        Some(preset) => {
            println!("Voice: {voice_name}");

            let input_sc: cpal::StreamConfig = input_config.into();
            let output_sc: cpal::StreamConfig = output_config.into();

            let input_ring = HeapRb::<f32>::new(sample_rate as usize * 2);
            let (mut input_prod, mut input_cons) = input_ring.split();

            let output_ring = HeapRb::<f32>::new(sample_rate as usize * 2);
            let (mut output_prod, mut output_cons) = output_ring.split();

            std::thread::spawn(move || {
                let (mut pre_chain, pitch_semitones, formant_ratio, mut post_chain) =
                    build_sample_chains(&preset.effects, sample_rate as f32);

                let mut shifter = PitchShifter::new(sample_rate, pitch_semitones);
                if formant_ratio != 1.0 {
                    shifter.set_formant_scale(formant_ratio);
                }
                let block_size = shifter.block_size();

                let mut input_block = vec![0.0f32; block_size];
                let mut output_block = vec![0.0f32; block_size];
                let mut filled = 0;

                loop {
                    while filled < block_size {
                        match input_cons.try_pop() {
                            Some(s) => {
                                let s = pre_chain.iter_mut().fold(s, |s, p| p.process(s));
                                input_block[filled] = s;
                                filled += 1;
                            }
                            None => {
                                std::thread::sleep(Duration::from_micros(50));
                            }
                        }
                    }
                    filled = 0;

                    shifter.shift(&input_block, &mut output_block);

                    for &s in &output_block {
                        let s = post_chain.iter_mut().fold(s, |s, p| p.process(s));
                        let _ = output_prod.try_push(s);
                    }
                }
            });

            let _input_stream = input_device.build_input_stream(
                input_sc,
                move |input: &[f32], _| {
                    for frame in input.chunks(input_channels) {
                        let mono = frame.iter().sum::<f32>() / frame.len() as f32;
                        let _ = input_prod.try_push(mono);
                    }
                },
                |e| eprintln!("Input error: {e}"),
                None,
            )?;

            let _output_stream = output_device.build_output_stream(
                output_sc,
                move |output: &mut [f32], _| {
                    for frame in output.chunks_mut(output_channels) {
                        let s = output_cons.try_pop().unwrap_or(0.0);
                        for ch in frame { *ch = s; }
                    }
                },
                |e| eprintln!("Output error: {e}"),
                None,
            )?;

            _input_stream.play()?;
            _output_stream.play()?;

            write_pid(voice_name)?;
            println!("Mimik is live [{}]", voice_name);
            println!("Speak into your microphone. Ctrl+C to stop.");
            loop { std::thread::sleep(Duration::from_secs(1)); }
        }
    }
}

fn write_pid(voice: &str) -> Result<()> {
    let content = format!("{}\n{}\n", std::process::id(), voice);
    fs::write(PID_FILE, content).context("Could not write PID file")
}

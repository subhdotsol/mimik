use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};
use std::time::Duration;

pub fn run_passthrough() -> Result<()> {
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
        .context("BlackHole 2ch was not found. Install it from existential.audio/blackhole")?;

    let input_desc = input_device.description()?;
    let output_desc = output_device.description()?;
    println!("Input:  {}", input_desc.name());
    println!("Output: {}", output_desc.name());

    let input_config = input_device.default_input_config()?;
    let output_config = output_device.default_output_config()?;

    let input_channels = input_config.channels() as usize;
    let output_channels = output_config.channels() as usize;
    let input_sample_rate = input_config.sample_rate();
    let output_sample_rate = output_config.sample_rate();

    println!(
        "Input:  {} ch @ {} Hz  {:?}",
        input_channels,
        input_sample_rate,
        input_config.sample_format()
    );
    println!(
        "Output: {} ch @ {} Hz  {:?}",
        output_channels,
        output_sample_rate,
        output_config.sample_format()
    );

    if input_config.sample_format() != cpal::SampleFormat::F32 {
        anyhow::bail!(
            "Input format {:?} is not supported yet",
            input_config.sample_format()
        );
    }

    if output_config.sample_format() != cpal::SampleFormat::F32 {
        anyhow::bail!(
            "Output format {:?} is not supported yet",
            output_config.sample_format()
        );
    }

    if input_sample_rate != output_sample_rate {
        anyhow::bail!(
            "Sample rate mismatch: input {} Hz vs output {} Hz",
            input_sample_rate,
            output_sample_rate
        );
    }

    // Ring buffer stores mono frames. 48_000 = 1 second headroom.
    let ring = HeapRb::<f32>::new(48_000);
    let (mut producer, mut consumer) = ring.split();

    // Mix all input channels down to mono, push one sample per frame.
    let input_stream = input_device.build_input_stream(
        input_config.into(),
        move |input: &[f32], _| {
            for frame in input.chunks(input_channels) {
                let mono = frame.iter().sum::<f32>() / frame.len() as f32;
                let _ = producer.try_push(mono);
            }
        },
        |error| {
            eprintln!("Input stream error: {error}");
        },
        None,
    )?;

    // Pop one mono sample per frame and copy it to every output channel.
    let output_stream = output_device.build_output_stream(
        output_config.into(),
        move |output: &mut [f32], _| {
            for frame in output.chunks_mut(output_channels) {
                let sample = consumer.try_pop().unwrap_or(0.0);
                for ch in frame {
                    *ch = sample;
                }
            }
        },
        |error| {
            eprintln!("Output stream error: {error}");
        },
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    println!("Mimik is live.");
    println!("Speak into your microphone.");
    println!("Press Ctrl+C to stop.");

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

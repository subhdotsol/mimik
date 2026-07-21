use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};
use std::{time::Duration};

pub const PID_FILE: &str = "/tmp/mimik.pid";

pub fn run(voice: Option<&str>) -> Result<()> {
    let _ = voice;

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

    println!("Mimik is live [clean]");
    println!("Speak into your microphone. Ctrl+C to stop.");
    loop { std::thread::sleep(Duration::from_secs(1)); }
}

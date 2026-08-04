use anyhow::{Context, Ok, Result};
use cpal::traits::{DeviceTrait, HostTrait};

pub fn print_devices() -> Result<()> {
    let host = cpal::default_host();

    println!("Audio host: {}", host.id().name());
    println!();

    print_input_devices(&host)?;
    println!();
    print_output_devices(&host)?;

    Ok(())
}

pub fn print_input_devices(host: &cpal::Host) -> Result<()> {
    let default_device_name = host
        .default_input_device()
        .and_then(|device: cpal::Device| device.description().ok().map(|d| d.name().to_string()));

    println!("Input devices");

    let devices = host
        .input_devices()
        .context("Failed to enumerate input devices")?;

    let mut found_device = false;

    for (index, device) in devices.enumerate() {
        found_device = true;

        let device: cpal::Device = device;
        let name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| "Unknown input device".to_string());

        let default_marker = if default_device_name.as_deref() == Some(name.as_str()) {
            " [default]"
        } else {
            ""
        };

        println!("  {index}: {name}{default_marker}");

        match device.default_input_config() {
            std::result::Result::Ok(config) => {
                println!(
                    "     channels: {}, sample rate: {} Hz, format: {:?}",
                    config.channels(),
                    config.sample_rate(),
                    config.sample_format(),
                );
            }
            Err(error) => {
                println!("     configuration unavailable: {error}");
            }
        }
    }

    if !found_device {
        println!("  No input devices found");
    }

    Ok(())
}

pub fn print_output_devices(host: &cpal::Host) -> Result<()> {
    let default_device_name = host
        .default_output_device()
        .and_then(|device: cpal::Device| device.description().ok().map(|d| d.name().to_string()));

    println!("Output devices");

    let devices = host
        .output_devices()
        .context("Failed to enumerate output devices")?;

    let mut found_device = false;

    for (index, device) in devices.enumerate() {
        found_device = true;

        let device: cpal::Device = device;
        let name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| "Unknown output device".to_string());

        let default_marker = if default_device_name.as_deref() == Some(name.as_str()) {
            " [default]"
        } else {
            ""
        };

        println!("  {index}: {name}{default_marker}");

        match device.default_output_config() {
            std::result::Result::Ok(config) => {
                println!(
                    "     channels: {}, sample rate: {} Hz, format: {:?}",
                    config.channels(),
                    config.sample_rate(),
                    config.sample_format(),
                );
            }
            Err(error) => {
                println!("     configuration unavailable: {error}");
            }
        }
    }

    if !found_device {
        println!("  No output devices found");
    }

    Ok(())
}

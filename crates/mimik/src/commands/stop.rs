use anyhow::Result;
use mimik_audio::PID_FILE;
use std::{fs, path::Path, thread, time::Duration};

pub fn run() -> Result<()> {
    if !Path::new(PID_FILE).exists() {
        println!("Mimik is not running.");
        return Ok(());
    }

    let content = fs::read_to_string(PID_FILE)?;
    let mut lines = content.lines();
    let pid: u32 = match lines.next().and_then(|l| l.parse().ok()) {
        Some(p) => p,
        None => {
            println!("Mimik is not running (corrupt PID file).");
            let _ = fs::remove_file(PID_FILE);
            return Ok(());
        }
    };
    let voice = lines.next().unwrap_or("clean");

    let alive = std::process::Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !alive {
        println!("Mimik is not running.");
        let _ = fs::remove_file(PID_FILE);
        return Ok(());
    }

    println!("Stopping voice '{}' (PID {pid})...", voice);

    std::process::Command::new("kill")
        .arg(pid.to_string())
        .status()?;

    // Wait up to 2 s for the process to exit.
    for _ in 0..20 {
        thread::sleep(Duration::from_millis(100));
        let still_alive = std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !still_alive {
            break;
        }
    }

    let _ = fs::remove_file(PID_FILE);
    println!("Stopped.");

    Ok(())
}

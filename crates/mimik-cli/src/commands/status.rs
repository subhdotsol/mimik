use anyhow::Result;
use mimik_audio::PID_FILE;
use std::{fs, path::Path};

pub fn run() -> Result<()> {
    if !Path::new(PID_FILE).exists() {
        println!("Status: stopped");
        return Ok(());
    }

    let content = fs::read_to_string(PID_FILE)?;
    let mut lines = content.lines();
    let pid: u32 = match lines.next().and_then(|l| l.parse().ok()) {
        Some(p) => p,
        None => {
            println!("Status: stopped (corrupt PID file)");
            return Ok(());
        }
    };
    let voice = lines.next().unwrap_or("clean");

    let alive = std::process::Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if alive {
        println!("Status: running");
        println!("Voice:  {voice}");
        println!("PID:    {pid}");
    } else {
        println!("Status: stopped");
        let _ = fs::remove_file(PID_FILE);
    }

    Ok(())
}

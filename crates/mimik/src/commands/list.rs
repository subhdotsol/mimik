use anyhow::Result;
use mimik_core::voice::{load_manifest, voices_dir};
use std::fs;

pub fn run() -> Result<()> {
    let voices_dir = voices_dir();

    let mut entries: Vec<_> = fs::read_dir(&voices_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    println!("Installed voices");
    println!();
    println!("{:<12} {:<12} {}", "NAME", "ENGINE", "VERSION");

    for entry in entries {
        if let Ok(manifest) = load_manifest(&entry.path()) {
            println!(
                "{:<12} {:<12} {}",
                manifest.id, manifest.engine, manifest.version
            );
        }
    }

    Ok(())
}

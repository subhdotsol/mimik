use anyhow::Result;
use mimik_core::voice::load_manifest;
use std::{fs, path::Path};

pub fn run() -> Result<()> {
    let voices_dir = Path::new("assets/voices");

    let mut entries: Vec<_> = fs::read_dir(voices_dir)?
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

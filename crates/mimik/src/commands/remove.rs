use anyhow::{Context, Result};
use mimik_core::voice::load_manifest;
use std::{fs, path::Path};

const VOICES_DIR: &str = "assets/voices";

pub fn run(voice: &str) -> Result<()> {
    let dir = Path::new(VOICES_DIR).join(voice);

    if !dir.exists() {
        anyhow::bail!(
            "Voice '{voice}' is not installed.\n\
             Run `mimik list` to see installed voices."
        );
    }

    let manifest = load_manifest(&dir)
        .context("Could not read manifest for this voice")?;

    if manifest.engine == "built-in" {
        anyhow::bail!(
            "'{voice}' is a built-in voice and cannot be removed."
        );
    }

    fs::remove_dir_all(&dir)
        .with_context(|| format!("Failed to remove {}", dir.display()))?;

    println!("Removed voice '{}' ({}).", manifest.name, manifest.id);

    Ok(())
}

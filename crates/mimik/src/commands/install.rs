use anyhow::{Context, Result};
use mimik_core::voice::{load_manifest, voices_dir};
use std::{fs, path::Path};

pub fn run(path: &str) -> Result<()> {
    let src = Path::new(path);

    if !src.exists() {
        anyhow::bail!("Path not found: {path}");
    }
    if !src.is_dir() {
        anyhow::bail!(
            "Expected a directory containing manifest.toml.\n\
             Zip-based .mimikpack archives are not yet supported."
        );
    }

    let manifest = load_manifest(src)
        .context("Invalid voice pack: missing or malformed manifest.toml")?;

    let dest = voices_dir().join(&manifest.id);

    if dest.exists() {
        anyhow::bail!(
            "Voice '{}' is already installed.\n\
             Run `mimik remove {}` first if you want to replace it.",
            manifest.id,
            manifest.id
        );
    }

    copy_dir(src, &dest)
        .with_context(|| format!("Failed copying files to {}", dest.display()))?;

    println!("Installed:   {} ({})", manifest.name, manifest.id);
    println!("Engine:      {}", manifest.engine);
    println!("Version:     {}", manifest.version);
    if let Some(desc) = &manifest.description {
        println!("Description: {desc}");
    }
    println!("\nRun `mimik live {}` to use it.", manifest.id);

    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

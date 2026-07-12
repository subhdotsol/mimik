use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct VoiceManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub engine: String,
    pub description: Option<String>,
}

pub fn load_manifest(dir: &Path) -> Result<VoiceManifest> {
    let text = fs::read_to_string(dir.join("manifest.toml"))
        .with_context(|| format!("No manifest found at {}", dir.display()))?;
    toml::from_str(&text).context("Invalid voice manifest")
}

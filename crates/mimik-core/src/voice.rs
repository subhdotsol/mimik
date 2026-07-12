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

/// One entry in the `[[effects]]` array of a preset.toml.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {
    NoiseGate { threshold_db: f32 },
    PitchShift { semitones: f32 },
    FormantShift { ratio: f32 },
    HighPass { frequency_hz: f32 },
    Equalizer { frequency_hz: f32, gain_db: f32, q: f32 },
    Compressor { threshold_db: f32, ratio: f32, attack_ms: f32, release_ms: f32 },
    Limiter { ceiling_db: f32 },
    Vibrato { rate_hz: f32, depth_cents: f32, mix: f32 },
    DeEsser { frequency_hz: f32, threshold_db: f32 },
    Chorus { rate_hz: f32, depth_ms: f32, mix: f32 },
    Reverb { room_size: f32, mix: f32, decay_ms: f32 },
}

pub fn load_manifest(dir: &Path) -> Result<VoiceManifest> {
    let text = fs::read_to_string(dir.join("manifest.toml"))
        .with_context(|| format!("No manifest found at {}", dir.display()))?;
    toml::from_str(&text).context("Invalid voice manifest")
}

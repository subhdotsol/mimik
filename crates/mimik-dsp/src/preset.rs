#[derive(Debug, Clone)]
pub struct VoicePreset {
    pub name: String,
    pub pitch_semitones: f32,
    pub output_gain: f32,
}

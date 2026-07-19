use crate::preset::VoicePreset;

pub fn preset() -> VoicePreset {
    VoicePreset {
        name: "tom".to_string(),
        pitch_semitones: 7.0,
        output_gain: 0.85,
    }
}

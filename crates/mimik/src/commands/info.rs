use mimik_core::voice::{load_dsp_preset, load_manifest, voices_dir, Effect};

pub fn run(voice: &str) {
    let voice_dir = voices_dir().join(voice);

    let manifest = match load_manifest(&voice_dir) {
        Ok(m) => m,
        Err(_) => {
            println!("Unknown voice: '{voice}'");
            println!("Run 'mimik list' to see installed voices.");
            return;
        }
    };

    println!("Name:        {}", manifest.name);
    println!("ID:          {}", manifest.id);
    println!("Engine:      {}", manifest.engine);
    println!("Version:     {}", manifest.version);
    if let Some(desc) = &manifest.description {
        println!("Description: {desc}");
    }

    if manifest.engine == "dsp" {
        if let Ok(preset) = load_dsp_preset(&voice_dir) {
            for effect in &preset.effects {
                match effect {
                    Effect::PitchShift { semitones } => {
                        println!("Pitch:       +{semitones} semitones");
                    }
                    Effect::FormantShift { ratio } => {
                        println!("Formant:     ×{ratio}");
                    }
                    Effect::Limiter { ceiling_db } => {
                        println!("Limiter:     {ceiling_db} dB");
                    }
                    Effect::Compressor { threshold_db, ratio, .. } => {
                        println!("Compressor:  {threshold_db} dB  {ratio}:1");
                    }
                    _ => {}
                }
            }
        }
    }

    println!("Status:      installed");
}

use anyhow::Result;

pub fn run(voice: Option<String>) -> Result<()> {
    if let Some(voice) = voice {
        println!("Voice '{voice}' is not implemented yet.");
        println!("Starting clean passthrough instead.");
    }

    mimik_audio::pipeline::run_passthrough()
}

use anyhow::Result;

pub fn run(voice: Option<String>) -> Result<()> {
    mimik_audio::pipeline::run(voice.as_deref())
}

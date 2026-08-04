use anyhow::Result;

pub fn run() -> Result<()> {
    mimik_audio::devices::print_devices()
}

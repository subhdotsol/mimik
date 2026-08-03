pub fn run(voice: Option<String>) {
    match voice {
        Some(voice) => {
            println!("Starting live microphone processing...");
            println!("Voice: {voice}");
        }
        None => {
            println!("Starting live microphone processing...");
            println!("Voice: clean");
        }
    }
}

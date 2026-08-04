use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mimik")]
#[command(version)]
#[command(about = "A real-time programmable voice changer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List available input and output audio devices
    Devices,

    /// Start live microphone processing
    Live {
        /// Voice to apply, such as robot or demon
        voice: Option<String>,
    },

    /// List installed voices
    List,

    /// Search downloadable voices
    Search {
        /// Name or keyword to search for
        voice: String,
    },

    /// Install a voice
    Install {
        /// Voice name or local pack path
        voice: String,
    },

    /// Remove an installed voice
    Remove {
        /// Name of the installed voice
        voice: String,
    },

    /// Show detailed information about a voice
    Info {
        /// Name of the voice
        voice: String,
    },

    /// Show the current Mimik status
    Status,

    /// Stop voice processing and return to clean passthrough
    Stop,
}

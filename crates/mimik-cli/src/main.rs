mod cli;
mod commands;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Devices => commands::devices::run(),

        Commands::Live { voice } => commands::live::run(voice),

        Commands::List => commands::list::run(),

        Commands::Search { voice } => commands::search::run(&voice),

        Commands::Install { voice } => commands::install::run(&voice),

        Commands::Remove { voice } => commands::remove::run(&voice),

        Commands::Info { voice } => commands::info::run(&voice),

        Commands::Status => commands::status::run(),

        Commands::Stop => commands::stop::run(),
    }

    Ok(())
}

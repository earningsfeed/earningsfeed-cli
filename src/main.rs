mod cli;
mod commands;
mod config;
mod error;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Command};

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Auth { action } => commands::auth::run(action),
        Command::Filings { action } => commands::filings::run(action).await,
        Command::Insider { action } => commands::insider::run(action).await,
        Command::Institutional { action } => commands::institutional::run(action).await,
        Command::Companies { action } => commands::companies::run(action).await,
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            e.exit_code()
        }
    }
}

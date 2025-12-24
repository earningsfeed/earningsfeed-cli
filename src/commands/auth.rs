//! Auth command implementation.

use std::io::{self, Write};

use crate::cli::AuthAction;
use crate::config::Config;
use crate::error::Result;

pub fn run(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => login(),
        AuthAction::Logout => logout(),
        AuthAction::Status => status(),
    }
}

fn login() -> Result<()> {
    print!("Enter API key: ");
    io::stdout().flush().unwrap();

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    let api_key = api_key.trim().to_string();

    if api_key.is_empty() {
        eprintln!("Error: API key cannot be empty");
        return Ok(());
    }

    let mut config = Config::load()?;
    config.api_key = Some(api_key);
    config.save()?;

    let path = Config::path()?;
    println!("API key saved to {}", path.display());
    Ok(())
}

fn logout() -> Result<()> {
    let mut config = Config::load()?;

    if config.api_key.is_none() {
        println!("Not logged in");
        return Ok(());
    }

    config.api_key = None;
    config.save()?;

    println!("Logged out");
    Ok(())
}

fn status() -> Result<()> {
    let config = Config::load()?;

    match &config.api_key {
        Some(key) => {
            // Show masked key: first 6 chars + ... + last 3 chars
            let masked = if key.len() > 9 {
                format!("{}...{}", &key[..6], &key[key.len() - 3..])
            } else {
                "***".to_string()
            };
            println!("Authenticated (key: {})", masked);
        }
        None => {
            println!("Not authenticated. Run 'earningsfeed auth login' to log in.");
        }
    }

    Ok(())
}

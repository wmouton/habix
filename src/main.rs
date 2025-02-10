use clap::Parser;
use std::io::{self, Write}; // Add this line
use std::path::Path; // Add this line
mod auth;
mod tasks;
mod notifications;
mod cli;
mod errors;

use cli::{Cli, Commands};
use crate::errors::AppError;

fn setup() -> Result<(), AppError> {
    let mut user_id = String::new();
    let mut api_token = String::new();

    print!("Enter your Habitica User ID: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_id)?;

    print!("Enter your Habitica API Token: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut api_token)?;

    // Trim newline characters from input
    let user_id = user_id.trim().to_string();
    let api_token = api_token.trim().to_string();

    auth::save_credentials(&user_id, &api_token)?;
    println!("✅ Setup complete! Credentials saved in .env file.");

    Ok(())
}

fn run() -> Result<(), AppError> {
    let env_file = Path::new(".env");

    if !env_file.exists() {
        println!("❌ Error: .env file not found. Please run 'habix setup' to configure your credentials.");
        return Ok(());
    }

    auth::load_env()?;
    let (user_id, api_token) = match auth::get_credentials() {
        Ok(creds) => creds,
        Err(_) => {
            println!("❌ Error: Missing or invalid credentials in .env file. Please run 'habix setup' to configure your credentials.");
            return Ok(());
        }
    };

    let tasks = tasks::fetch_tasks(&user_id, &api_token)?;
    let pending = tasks::get_pending_tasks(tasks);

    if pending.is_empty() {
        notifications::notify("Habix Reminder - Habitica Tasks", "✅ You're all caught up! ✅")?;
    } else {
        let task_list = pending.join("\n");
        notifications::notify("Habix Reminder - Habitica Tasks", &format!("\n🔴 You have pending tasks 🔴:\n\n{}", task_list))?;
    }

    Ok(())
}

fn clean() -> Result<(), AppError> {
    auth::clean_credentials()?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Run => run(),
        Commands::Setup => setup(),
        Commands::Clean => clean(),
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

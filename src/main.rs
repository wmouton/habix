use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::process::Command; // Add this import

/// Habitica Notifier
#[derive(Parser)]
#[command(name = "habix")]
#[command(about = "Fetches pending Habitica tasks and sends desktop notifications")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the task checker
    Run,
    /// Sets up Habitica API credentials
    Setup,
}

#[derive(Deserialize)]
struct HabiticaResponse {
    data: Vec<Task>,
}

#[derive(Deserialize)]
struct Task {
    text: String,
    #[serde(rename = "type")]
    task_type: String,
    completed: Option<bool>,
}

/// Returns the path to `~/habix-keys/habix.md`
fn get_credentials_path() -> String {
    let home_dir = env::var("HOME").expect("Could not determine home directory");
    format!("{}/habix-keys/habix.md", home_dir)
}

/// Reads Habitica credentials from `~/habix-keys/habix.md`
fn read_credentials() -> Result<(String, String), String> {
    let path = get_credentials_path();
    let content = fs::read_to_string(&path).map_err(|_| "Credentials file not found. Run `habix setup`.")?;
    let lines: Vec<&str> = content.lines().collect();

    if lines.len() < 2 {
        return Err("Invalid credentials format. Run `habix setup` again.".to_string());
    }

    let user_id = lines[0].trim().to_string();
    let api_token = lines[1].trim().to_string();
    Ok((user_id, api_token))
}

/// Writes Habitica credentials to `~/habix-keys/habix.md`
fn save_credentials(user_id: &str, api_token: &str) -> Result<(), String> {
    let path = get_credentials_path();
    let dir_path = Path::new(&path).parent().unwrap();

    // Create the directory if it does not exist
    fs::create_dir_all(dir_path).map_err(|e| format!("Failed to create directory: {}", e))?;

    // Save credentials in a markdown-like format
    let content = format!("{}\n{}\n", user_id, api_token);
    fs::write(path, content).map_err(|e| format!("Failed to save credentials: {}", e))?;

    Ok(())
}

/// Fetches Habitica tasks
fn fetch_tasks() -> Result<Vec<Task>, String> {
    let (user_id, api_token) = read_credentials()?;

    let url = "https://habitica.com/api/v3/tasks/user";
    let client = Client::new();

    let response = client
        .get(url)
        .header("x-api-user", user_id)
        .header("x-api-key", api_token)
        .header("Content-Type", "application/json")
        .send()
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    let json: HabiticaResponse = response.json().map_err(|e| format!("Invalid JSON: {}", e))?;
    Ok(json.data)
}

/// Filters pending tasks
fn get_pending_tasks(tasks: Vec<Task>) -> Vec<String> {
    tasks
        .into_iter()
        .filter(|task| (task.task_type == "daily" || task.task_type == "todo") && !task.completed.unwrap_or(false))
        .map(|task| task.text)
        .collect()
}

/// Sends a notification using `notify-send`
fn notify(title: &str, message: &str) {
    let status = Command::new("notify-send")
        .arg(title)
        .arg(message)
        .status();

    if let Err(e) = status {
        eprintln!("Failed to send notification: {}", e);
    }
}

/// Prompts for credentials and saves them
fn setup() {
    let mut user_id = String::new();
    let mut api_token = String::new();

    print!("Enter Habitica User ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_id).unwrap();
    let user_id = user_id.trim();

    print!("Enter Habitica API Key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut api_token).unwrap();
    let api_token = api_token.trim();

    if save_credentials(user_id, api_token).is_ok() {
        println!("✅ Setup complete! Credentials saved at ~/habix-keys/habix.md");
    } else {
        eprintln!("❌ Failed to save credentials.");
        process::exit(1);
    }
}

/// Runs the main task checker
fn run() {
    match fetch_tasks() {
        Ok(tasks) => {
            let pending = get_pending_tasks(tasks);
            if pending.is_empty() {
                notify("Habix Reminder - Habitica", "✅ You're all caught up! ✅");
            } else {
                let task_list = pending.join("\n");
                notify("Habix Reminder - Habitica", &format!("\n🔴 You have pending tasks 🔴:\n\n{}", task_list));
            }
        }
        Err(e) => {
            notify("Error", &format!("Failed to fetch tasks: {}", e));
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run => run(),
        Commands::Setup => setup(),
    }
}

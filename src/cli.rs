use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the task checker
    Run,
    /// Setup credentials
    Setup,
    /// Delete the .env file
    Clean,
}

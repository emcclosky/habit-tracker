use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "habit-tracker")]
// These attributes automatically get filled from the Cargo.toml file
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new habit
    Add { name: String },
    /// List all habits and their streaks
    List,
    /// Mark a habit as complete
    Complete { name: String },
}

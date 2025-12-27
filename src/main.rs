use clap::{Parser, Subcommand};
use colored::*;

/// Sprout: Productivity with a pulse.
#[derive(Parser)]
#[command(name = "Sprout")]
#[command(author = "SuperCodeAurora")]
#[command(version = "1.0")]
#[command(about = "A bio-digital productivity pet.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Plant a new task (Add to list)
    Add {
        /// The task description
        task: String,
    },
    /// Complete a task (Feed the Sprout)
    Done {
        /// How many tasks did you do? (Default: 1)
        #[arg(default_value_t = 1)]
        amount: u32,
    },
    /// Check your Sprout's health and ASCII art
    Status,
    /// Freeze Sprout for vacation (Cost: 15 coins/day)
    Freeze {
        /// Number of days (Max 3)
        #[arg(short, long)]
        days: u8,
    },
    /// Emergency Resuscitation (Only works in COMA)
    Cpr,
}

fn main() {
    let cli = Cli::parse();

    // 🌿 The Skeleton Logic Switch
    match &cli.command {
        Commands::Add { task } => {
            println!("{} {}", "🌱 Seed planted:".green(), task);
            // TODO: Save to JSON
        }
        Commands::Done { amount } => {
            println!("{} You fed Sprout {} nutrient(s).", "💰".yellow(), amount);
            // TODO: Update Coins += amount
        }
        Commands::Status => {
            // Placeholder Art for "Day 1 Hunger"
            println!("{}", "(︶︿︶)".red().bold());
            println!("Status: {} | Coins: {}", "HUNGRY".red(), "0".yellow()); 
            println!("Type 'sprout done' to feed.");
        }
        Commands::Freeze { days } => {
            if *days > 3 {
                println!("{} Biology cannot sustain > 3 days freeze.", "🚫 ERROR:".red());
            } else {
                println!("{} Sprout frozen for {} days. (Cost: {} coins)", "🧊".cyan(), days, days * 15);
            }
        }
        Commands::Cpr => {
            println!("{} CPR Kit deployed. 50 Coins deducted.", "🚑".red());
        }
    }
}

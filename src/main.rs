mod state;
mod assets; // <--- This is the key line that finds your new Art Gallery

use clap::{Parser, Subcommand};
use colored::*;
use chrono::{Utc, Duration};

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
        amount: u64,
    },
    /// Check your Sprout's health and ASCII art
    Status,
    /// Freeze Sprout for vacation (Cost: 15 coins/day)
    Freeze {
        /// Number of days (Max 3)
        #[arg(short, long)]
        days: i64,
    },
    /// Emergency Resuscitation (Only works in COMA)
    Cpr,
}

fn main() {
    let cli = Cli::parse();
    
    // 🧠 LOAD THE BRAIN
    // This loads the JSON file to see how many coins you have
    let mut sprout = state::SproutState::load();

    match &cli.command {
        Commands::Add { task } => {
            println!("{} {}", "🌱 Seed planted:".green(), task);
        }
        Commands::Done { amount } => {
            sprout.feed(*amount);
            println!("{} You fed Sprout {} nutrient(s).", "🍎".red(), amount);
            println!("Total Coins: {}", sprout.coins.to_string().yellow());
        }
        Commands::Status => {
            // 1. Get the ASCII face based on health AND coins (Evolution)
            let art = sprout.get_status_ascii();
            
            // 2. Print the Art
            println!("\n{}\n", art.cyan().bold());
            
            // 3. Print Stats
            if sprout.is_coma {
                println!("{}", "❌ SYSTEM FAILURE: COMA STATE".on_red().white().bold());
                println!("Type 'sprout cpr' to attempt resuscitation.");
            } else {
                println!("Coins: {} 🟡", sprout.coins.to_string().yellow().bold());
                
                if sprout.is_frozen {
                     println!("{}", "❄️ STATUS: FROZEN".blue());
                } else {
                     // Calculate mood based on hunger
                     println!("{}", "✨ STATUS: ACTIVE".green());
                }
            }
        }
        Commands::Freeze { days } => {
            let cost = *days as u64 * 15;
            
            if *days > 3 {
                println!("{} Biology cannot sustain > 3 days freeze.", "🚫 ERROR:".red());
            } else if sprout.coins < cost {
                println!("{} You need {} coins to freeze for {} days. You have {}.", "💸".red(), cost, days, sprout.coins);
            } else {
                sprout.coins -= cost;
                sprout.is_frozen = true;
                sprout.frozen_until = Some(Utc::now() + Duration::days(*days));
                sprout.save();
                println!("{} Sprout frozen for {} days. (Cost: {} coins)", "🧊".cyan(), days, cost);
                println!("Remaining Coins: {}", sprout.coins);
            }
        }
        Commands::Cpr => {
            if sprout.perform_cpr() {
                println!("{} CPR SUCCESSFUL! Life signs detected.", "⚡".yellow());
            } else {
                println!("{} CPR FAILED. You need 50 coins.", "💀".red());
            }
        }
    }
}

mod state;
mod assets;

use clap::{Parser, Subcommand};
use colored::*;
use chrono::{Utc, Duration};

#[derive(Parser)]
#[command(name = "Sprout")]
#[command(author = "SuperCodeAurora")]
#[command(version = "1.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a task to your list
    Add { task: String },
    /// Complete the top task!
    Done,
    /// Check health and view tasks
    Status,
    /// Freeze Sprout (Vacation Mode)
    Freeze { #[arg(short, long)] days: i64 },
    Cpr,
}

fn main() {
    let cli = Cli::parse();
    let mut sprout = state::SproutState::load();

    match &cli.command {
        Commands::Add { task } => {
            sprout.add_task(task.clone());
            println!("{} {}", "🌱 Task Planted:".green(), task);
            println!("Current Tasks: {}", sprout.tasks.len());
        }
        Commands::Done => {
            // Try to complete the top task
            if sprout.complete_task() {
                println!("{} Task Completed! (+10 Coins)", "🍎".red());
                println!("Total Coins: {}", sprout.coins.to_string().yellow());
            }
        }
        Commands::Status => {
            let art = sprout.get_status_ascii();
            println!("\n{}\n", art.cyan().bold());
            
            if sprout.is_coma {
                println!("{}", "❌ SYSTEM FAILURE: COMA STATE".on_red().white().bold());
                println!("Type 'sprout cpr' to attempt resuscitation.");
            } else {
                println!("Coins: {} 🟡", sprout.coins.to_string().yellow().bold());
                
                // 🆕 PRINT THE TODO LIST
                println!("\n📝 {}:", "Active Tasks".underline());
                if sprout.tasks.is_empty() {
                    println!("   (No tasks planted. Use 'sprout add' to grow!)");
                } else {
                    for (i, task) in sprout.tasks.iter().enumerate() {
                        println!("   {}. {}", i + 1, task);
                    }
                }
            }
        }
        Commands::Freeze { days } => {
            let cost = *days as u64 * 15;
            if sprout.coins < cost {
                println!("{} Need {} coins. You have {}.", "💸".red(), cost, sprout.coins);
            } else {
                sprout.coins -= cost;
                sprout.is_frozen = true;
                sprout.frozen_until = Some(Utc::now() + Duration::days(*days));
                sprout.save();
                println!("{} Sprout frozen for {} days.", "🧊".cyan(), days);
            }
        }
        Commands::Cpr => {
            if sprout.perform_cpr() {
                println!("{} CPR SUCCESSFUL!", "⚡".yellow());
            } else {
                println!("{} CPR FAILED. Need 50 coins.", "💀".red());
            }
        }
    }
}

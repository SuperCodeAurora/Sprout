mod state;
mod assets;

use clap::{Parser, Subcommand};
use colored::*;
use chrono::{Utc, Duration};
use std::fs;
use std::os::unix::fs::PermissionsExt; // UNIX only, handle Windows appropriately if needed
use std::path::Path;

#[derive(Parser)]
#[command(name = "Sprout")]
#[command(author = "SuperCodeAurora")]
#[command(version = "1.2-VIRAL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a task manually
    Add { task: String },
    /// Feed Sprout (Manual or via Hook)
    Done { 
        #[arg(short, long)]
        git: bool, // Hidden flag for git hooks
        #[arg(short, long)]
        message: Option<String> 
    },
    /// Check status & Health
    Status,
    /// Vacation Mode
    Freeze { #[arg(short, long)] days: i64 },
    /// Save your dying plant
    Cpr,
    /// 🔌 NEW: Install Git Hook to current repo
    InitHook,
}

// 🔥 HELPER: Visual Progress Bar
fn draw_progress_bar(current: u64, max: u64, width: usize) -> String {
    let filled = (current as f64 / max as f64 * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    format!(
        "[{}{}] {}/{}", 
        "█".repeat(filled).green(), 
        "-".repeat(empty).black().bright(),
        current, max
    )
}

// 🔥 HELPER: Get Next Evolution Threshold
fn get_next_threshold(coins: u64) -> (u64, String) {
    if coins < 50 { (50, "Sapling".to_string()) }
    else if coins < 200 { (200, "Scholar".to_string()) }
    else if coins < 250 { (250, "Adult".to_string()) }
    else if coins < 1000 { (1000, "Hero".to_string()) }
    else if coins < 5000 { (5000, "Cyber-Ent".to_string()) }
    else { (10000, "King".to_string()) }
}

fn main() {
    let cli = Cli::parse();
    let mut sprout = state::SproutState::load();

    match &cli.command {
        Commands::InitHook => {
            // 🦠 VIRAL MECHANIC: The Infection
            // Installs a post-commit hook into the current .git folder
            let hook_path = Path::new(".git/hooks/post-commit");
            if !Path::new(".git").exists() {
                println!("{}", "❌ Not a git repository!".red());
                return;
            }
            
            let hook_script = r#"#!/bin/sh
echo "🌱 Sprout is watching..."
sprout done --git --message "Git Commit"
"#;
            fs::write(hook_path, hook_script).expect("Failed to write hook");
            
            // Make executable (Unix)
            let mut perms = fs::metadata(hook_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(hook_path, perms).unwrap();

            println!("{}", "✅ Sprout integrated into this Repo!".green().bold());
            println!("Every commit will now feed your Sprout automatically.");
        }

        Commands::Add { task } => {
            sprout.add_task(task.clone());
            println!("{} \"{}\"", "🌱 Seed Planted:".green(), task);
        }

        Commands::Done { git, message } => {
            // Logic: If Git mode, we don't need a task to exist. We just feed.
            let fed = if *git {
                sprout.coins += 10;
                sprout.last_fed = Utc::now();
                sprout.save();
                true
            } else {
                sprout.complete_task()
            };

            if fed {
                let source = if *git { "Git Commit" } else { "Task" };
                println!("{} {} Consumed! (+10 Coins)", "🍎".red(), source);
                
                // 🎉 LEVEL UP CHECK
                // (Simplified: You can add logic to compare old_level vs new_level)
                let (max, title) = get_next_threshold(sprout.coins);
                if sprout.coins >= max {
                     println!("\n{}\n", "✨ EVOLUTION IMMINENT! CHECK STATUS! ✨".on_yellow().black().bold());
                }
            }
        }

        Commands::Status => {
            let art = sprout.get_status_ascii(); // Now dynamic!
            
            // Clear screen for dramatic effect? Maybe just newlines.
            println!("\n{}\n", art.cyan().bold()); 
            
            if sprout.is_coma {
                println!("{}", "❌ SYSTEM FAILURE: COMA STATE".on_red().white().bold());
                println!("Type 'sprout cpr' to attempt resuscitation.");
            } else {
                // 📊 DRAW HUD
                let (next_target, next_rank) = get_next_threshold(sprout.coins);
                let bar = draw_progress_bar(sprout.coins, next_target, 20);
                
                println!("Rank: {}", sprout.coins.to_string().yellow().bold());
                println!("Next: {} ({})", next_rank.purple(), bar);
                
                // Check Cheater
                if sprout.is_cheater {
                    println!("\n{}", "⚠️  BIO-HAZARD DETECTED (INTEGRITY COMPROMISED)".red().blink());
                }

                println!("\n📝 {}:", "Active Tasks".underline());
                if sprout.tasks.is_empty() {
                    println!("   (Hungry... give me tasks or git commits)");
                } else {
                    // Show top 3 only to keep UI clean?
                    for (i, task) in sprout.tasks.iter().enumerate().take(5) {
                        println!("   {}. {}", i + 1, task);
                    }
                    if sprout.tasks.len() > 5 {
                        println!("   ...and {} more.", sprout.tasks.len() - 5);
                    }
                }
            }
        }

        Commands::Freeze { days } => {
            // ... (Logic kept same)
            let cost = *days as u64 * 15;
            if sprout.coins < cost {
                println!("{} Need {} coins.", "💸".red(), cost);
            } else {
                sprout.coins -= cost;
                sprout.is_frozen = true;
                sprout.frozen_until = Some(Utc::now() + Duration::days(*days));
                sprout.save();
                println!("{} Cryo-Stasis Active.", "🧊".cyan());
            }
        }

        Commands::Cpr => {
             if sprout.perform_cpr() {
                println!("{} PULSE DETECTED.", "⚡".yellow().bold());
            } else {
                println!("{} CPR FAILED. INSUFFICIENT FUNDS.", "💀".red());
            }
        }
    }
}

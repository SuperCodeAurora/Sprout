// src/ui.rs
use colored::*;
use crate::state::SproutState;

pub struct Dashboard;

impl Dashboard {
    // 🔥 CLEAR SCREEN (The "App" Feel)
    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    // 🎨 RENDER THE HUD
    pub fn render(state: &mut SproutState, art: &str) {
        Self::clear();

        let width = 60;
        let border_color = "blue";
        
        // 1. HEADER
        println!("{}", format!("╭{}╮", "─".repeat(width - 2)).color(border_color));
        println!(
            "{}  {:<35} {:>18} {}", 
            "│".color(border_color), 
            "🌱 SPROUT STATUS".bold().green(), 
            if state.is_coma { "[CRITICAL]".red().blink() } else { "[ONLINE]".green() },
            "│".color(border_color)
        );
        println!("{}", format!("├{}┤", "─".repeat(width - 2)).color(border_color));

        // 2. CONTENT (SIDE BY SIDE)
        // We need to split the ASCII art into lines and the Stats into lines
        // Then print them row by row.
        
        let art_lines: Vec<&str> = art.trim().lines().collect();
        let (next_target, next_rank) = crate::get_next_threshold(state.coins);
        let progress_str = crate::draw_progress_bar(state.coins, next_target, 15);
        
        // Prepare Stats Lines
        let stat_lines = vec![
            format!(""),
            format!("🪙  Coins: {}", state.coins.to_string().yellow().bold()),
            format!("🏷️  Rank:  {}", next_rank.cyan()),
            format!("🎯 Next:  {}", next_target),
            format!("📊 Prog:  {}", progress_str),
            format!(""),
        ];

        let max_lines = std::cmp::max(art_lines.len(), stat_lines.len());

        for i in 0..max_lines {
            let art_part = art_lines.get(i).unwrap_or(&"");
            let stat_part = stat_lines.get(i).unwrap_or(&"".to_string());
            
            // Dynamic padding ensures the right border stays aligned
            // Width - 2 (borders) - 20 (art width allocation) - 2 (padding)
            let art_width = 25; 
            let right_padding = width - 2 - art_width;

            println!(
                "{} {:<width$} {:<pad$} {}",
                "│".color(border_color),
                art_part,
                stat_part,
                "│".color(border_color),
                width = art_width,
                pad = right_padding - 1
            );
        }

        // 3. TASKS FOOTER
        println!("{}", format!("├{}┤", "─".repeat(width - 2)).color(border_color));
        println!(
            "{}  {:<54} {}", 
            "│".color(border_color), 
            "📝 PENDING TASKS".bold().white(), 
            "│".color(border_color)
        );
        
        if state.tasks.is_empty() {
             println!(
                "{}  {:<54} {}", 
                "│".color(border_color), 
                "(System Idle. Waiting for input...)", 
                "│".color(border_color)
            );
        } else {
             for (i, task) in state.tasks.iter().enumerate().take(5) {
                let clean_task = if task.len() > 45 { &task[0..45] } else { task };
                println!(
                    "{}  {}. {:<51} {}", 
                    "│".color(border_color), 
                    i + 1, 
                    clean_task, 
                    "│".color(border_color)
                );
            }
        }

        // 4. FOOTER
        println!("{}", format!("╰{}╯", "─".repeat(width - 2)).color(border_color));
        
        // 5. ALERTS
        if state.is_cheater {
            println!("\n{}", "⚠️  SECURITY ALERT: GENETIC MODIFICATION DETECTED".on_red().white().bold());
        }
    }
}

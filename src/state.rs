use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = ".sprout_state.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct SproutState {
    pub coins: u64,
    pub last_fed: DateTime<Utc>,
    pub is_frozen: bool,
    pub frozen_until: Option<DateTime<Utc>>,
    pub is_coma: bool,
    // 🆕 NEW: The Task List!
    pub tasks: Vec<String>, 
}

impl SproutState {
    pub fn new() -> Self {
        Self {
            coins: 0,
            last_fed: Utc::now(),
            is_frozen: false,
            frozen_until: None,
            is_coma: false,
            tasks: Vec::new(), // Start empty
        }
    }

    pub fn load() -> Self {
        if Path::new(SAVE_FILE).exists() {
            let content = fs::read_to_string(SAVE_FILE).unwrap_or_default();
            // If loading fails (e.g., old save format), start fresh
            serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(SAVE_FILE, json).expect("Unable to save state");
    }

    // 🆕 NEW: Add a task
    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
        self.save();
    }

    // 🆕 NEW: Complete a task (Feed & Remove)
    // Returns true if a task was actually removed
    pub fn complete_task(&mut self) -> bool {
        if self.is_coma {
            println!("🚫 Sprout is in a COMA. You must perform CPR!");
            return false;
        }
        
        if self.tasks.is_empty() {
            println!("🚫 No tasks to complete! Plant one first.");
            return false;
        }

        // Remove the oldest task (FIFO)
        let _completed = self.tasks.remove(0); 
        
        // Reward logic
        self.coins += 10; // 10 coins per task
        self.last_fed = Utc::now();
        
        if self.is_frozen {
            self.is_frozen = false;
            self.frozen_until = None;
            println!("🔥 THAWING COMPLETE.");
        }
        
        self.save();
        true
    }

    pub fn perform_cpr(&mut self) -> bool {
        if self.coins >= 50 {
            self.coins -= 50;
            self.is_coma = false;
            self.last_fed = Utc::now();
            self.save();
            return true;
        }
        false
    }

    // 🩺 Check Health & Evolution
    pub fn get_status_ascii(&mut self) -> String {
        let now = Utc::now();
        
        // 1. Frozen Check
        if self.is_frozen {
            if let Some(thaw_time) = self.frozen_until {
                if now > thaw_time {
                    self.is_frozen = false;
                    self.frozen_until = None;
                    self.save();
                    return "❄️ THAWING...".to_string();
                }
            }
            return crate::assets::FROZEN.to_string();
        }

        // 2. Coma Check (> 72 hours)
        let hours_since = now.signed_duration_since(self.last_fed).num_hours();
        if hours_since > 72 {
            if !self.is_coma {
                self.is_coma = true;
                self.save();
            }
            return crate::assets::COMA.to_string();
        }

        // 3. Evolution Logic
        if self.coins < 50 { return crate::assets::SEED.to_string(); }
        else if self.coins < 200 { return crate::assets::SAPLING_50.to_string(); }
        else if self.coins < 250 { return crate::assets::SCHOLAR_200.to_string(); }
        else if self.coins < 1000 { return crate::assets::ADULT_250.to_string(); }
        else if self.coins < 5000 { return crate::assets::HERO_1000.to_string(); }
        else if self.coins < 10000 { return crate::assets::CYBER_ENT_5000.to_string(); }
        else if self.coins < 15000 { return crate::assets::ACE_KING_10000.to_string(); }
        else { return crate::assets::GALACTIC_GOD_15000.to_string(); }
    }
}

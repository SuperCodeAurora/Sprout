use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
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
}

impl SproutState {
    // 🌱 Constructor: Create a new sprout if none exists
    pub fn new() -> Self {
        Self {
            coins: 0,
            last_fed: Utc::now(),
            is_frozen: false,
            frozen_until: None,
            is_coma: false,
        }
    }

    // 💾 Load from disk
    pub fn load() -> Self {
        if Path::new(SAVE_FILE).exists() {
            let content = fs::read_to_string(SAVE_FILE).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    // 💾 Save to disk
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(SAVE_FILE, json).expect("Unable to save state");
    }

    // 🍎 Feed mechanism
    pub fn feed(&mut self, amount: u64) {
        if self.is_coma {
            println!("🚫 Sprout is in a COMA. You must perform CPR!");
            return;
        }
        self.coins += amount;
        self.last_fed = Utc::now();
        // If it was frozen, feeding it wakes it up immediately
        if self.is_frozen {
            self.is_frozen = false;
            self.frozen_until = None;
            println!("🔥 THAWING COMPLETE. Metabolism active.");
        }
        self.save();
    }

    // 🚑 CPR mechanism
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

    // 🩺 Check Health (The Guilt Protocol)
    pub fn get_status_ascii(&mut self) -> String {
        let now = Utc::now();
        
        // Check if Frozen
        if self.is_frozen {
            if let Some(thaw_time) = self.frozen_until {
                if now > thaw_time {
                    // Time to wake up!
                    self.is_frozen = false;
                    self.frozen_until = None;
                    self.save();
                    return "(︶︿︶) Waking up... HUNGRY!".to_string();
                }
            }
            return "(🧊 ︶ v ︶ 🧊💤)".to_string();
        }

        // Check Time since last fed
        let hours_since = now.signed_duration_since(self.last_fed).num_hours();

        if hours_since < 24 {
            return "( ˘ ▽ ˘ ) Happy".to_string();
        } else if hours_since < 48 {
            return "(︶︿︶) Hungry".to_string();
        } else if hours_since < 72 {
            return "(Tn T) Panic".to_string();
        } else {
            if !self.is_coma {
                self.is_coma = true;
                self.save();
            }
            return "(x_x) COMA".to_string();
        }
    }
}

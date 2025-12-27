use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug)]
pub struct SproutState {
    pub coins: u64,
    pub last_fed: DateTime<Utc>,
    pub is_frozen: bool,
    pub frozen_until: Option<DateTime<Utc>>,
    pub is_coma: bool,
    pub tasks: Vec<String>,
    // 🛡️ Integrity Check
    #[serde(default)]
    pub integrity_hash: String, 
    #[serde(skip)] // Do not save this flag to file
    pub is_cheater: bool,
}

impl SproutState {
    // 🔥 HELPER: Get Global Path (Cross-Platform)
    fn get_state_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "supercodeaurora", "sprout") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                fs::create_dir_all(config_dir).unwrap_or_default();
            }
            return config_dir.join("state.json");
        }
        // Fallback to local if OS fails
        PathBuf::from(".sprout_state.json")
    }

    // 🔥 HELPER: Calculate Hash to detect manual edits
    fn calculate_hash(&self) -> String {
        let data = format!("{}:{}:{}", self.coins, self.last_fed, "SPROUT_SECRET_SALT_v1");
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn new() -> Self {
        let mut state = Self {
            coins: 0,
            last_fed: Utc::now(),
            is_frozen: false,
            frozen_until: None,
            is_coma: false,
            tasks: Vec::new(),
            integrity_hash: String::new(),
            is_cheater: false,
        };
        state.integrity_hash = state.calculate_hash();
        state
    }

    pub fn load() -> Self {
        let path = Self::get_state_path();
        
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            match serde_json::from_str::<SproutState>(&content) {
                Ok(mut state) => {
                    // 🕵️ DETECT CHEATING
                    let calculated = state.calculate_hash();
                    if state.integrity_hash != calculated {
                        println!("⚠️  GENETIC TAMPERING DETECTED. SPROUT DNA CORRUPTED.");
                        state.is_cheater = true;
                        // Optional: Penalty logic here
                    }
                    state
                },
                Err(_) => Self::new(),
            }
        } else {
            Self::new()
        }
    }

    pub fn save(&mut self) {
        let path = Self::get_state_path();
        // Recalculate hash before saving
        self.integrity_hash = self.calculate_hash();
        
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).expect("Unable to save state");
    }

    // ... (Your other logic: add_task, perform_cpr)

    // 🆕 Optimized complete_task (LIFO - Last In First Out)
    pub fn complete_task(&mut self) -> bool {
         if self.is_coma {
            println!("🚫 Sprout is in a COMA. You must perform CPR!");
            return false;
        }
        
        // Use POP instead of REMOVE(0) to align with "I just did this" mental model
        if let Some(_task) = self.tasks.pop() {
            self.coins += 10;
            self.last_fed = Utc::now();
            
            // Check Cheater Status for ASCII output later
            if self.is_cheater {
                println!("💀 Your Sprout eats the task... suspiciously.");
            }

            // ... Thaw logic ...
            self.save();
            return true;
        } else {
             println!("🚫 No tasks! Plant one first.");
             return false;
        }
    }
}

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

// 🔒 SECURITY CONFIG
const SALT: &str = "SPROUT_BIOLOGICAL_KEY_v2_DONOTTOUCH";
const MIN_FEED_INTERVAL_SECONDS: i64 = 30; // Can only feed once every 30s
const MAX_COINS_PER_DAY: u64 = 500; // Cap daily earnings

#[derive(Serialize, Deserialize, Debug)]
pub struct SproutState {
    pub coins: u64,
    pub last_fed: DateTime<Utc>,
    pub is_frozen: bool,
    pub frozen_until: Option<DateTime<Utc>>,
    pub is_coma: bool,
    pub tasks: Vec<String>,
    // 🛡️ Security Fields
    pub integrity_hash: String,
    pub daily_feed_count: u64,
    pub last_reset_day: DateTime<Utc>,
    #[serde(skip)]
    pub is_cheater: bool,
    #[serde(skip)]
    pub cheat_reason: String,
}

impl SproutState {
    // ... (get_state_path remains the same) ...

    fn calculate_hash(&self) -> String {
        // We include internal metrics in the hash so they can't be edited individually
        let data = format!("{}:{}:{}:{}:{}", 
            self.coins, 
            self.last_fed, 
            self.daily_feed_count, 
            self.is_coma, 
            SALT
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn load() -> Self {
        let path = Self::get_state_path();
        
        if path.exists() {
            // 1. READ (Now expects Base64 encoded string, not raw JSON)
            let encoded_content = fs::read_to_string(&path).unwrap_or_default();
            
            // 2. DECODE
            let decoded_bytes = general_purpose::STANDARD
                .decode(&encoded_content)
                .unwrap_or_default(); // Fallback if tampering broke encoding
            
            let json_str = String::from_utf8(decoded_bytes).unwrap_or_default();

            match serde_json::from_str::<SproutState>(&json_str) {
                Ok(mut state) => {
                    // 🕵️ SECURITY CHECK 1: Hash Validation
                    let calculated = state.calculate_hash();
                    if state.integrity_hash != calculated {
                        state.mark_cheater("DNA_MISMATCH");
                    }

                    // 🕵️ SECURITY CHECK 2: Time Travel (The Future-Fed Plant)
                    if state.last_fed > Utc::now() + Duration::minutes(5) {
                        state.mark_cheater("CHRONO_DISPLACEMENT");
                    }

                    state
                },
                Err(_) => Self::new(), // Corrupted save = Rebirth
            }
        } else {
            Self::new()
        }
    }

    pub fn save(&mut self) {
        let path = Self::get_state_path();
        self.integrity_hash = self.calculate_hash();
        
        let json = serde_json::to_string(&self).unwrap();
        
        // 🔒 ENCODE (Obfuscation layer)
        let encoded = general_purpose::STANDARD.encode(json);
        
        fs::write(path, encoded).expect("Unable to save state");
    }

    fn mark_cheater(&mut self, reason: &str) {
        self.is_cheater = true;
        self.cheat_reason = reason.to_string();
        // Punishment: Cap coins or visual shame? 
        // We do not reset coins here to avoid "Rage Quit", but we brand them.
    }

    // 🛡️ SECURE TASK COMPLETION
    pub fn complete_task(&mut self, is_git: bool) -> bool {
        let now = Utc::now();

        // 1. COMA CHECK
        if self.is_coma {
            println!("🚫 Sprout is in a COMA. CPR required.");
            return false;
        }

        // 2. RATE LIMIT (Digestion Speed)
        let seconds_since_fed = now.signed_duration_since(self.last_fed).num_seconds();
        if seconds_since_fed < MIN_FEED_INTERVAL_SECONDS {
            println!("🚫 Sprout is still digesting! Wait {}s.", MIN_FEED_INTERVAL_SECONDS - seconds_since_fed);
            return false;
        }

        // 3. DAILY CAP (Anti-Bot)
        // Reset day if needed
        if now.date_naive() > self.last_reset_day.date_naive() {
            self.daily_feed_count = 0;
            self.last_reset_day = now;
        }
        if self.daily_feed_count >= MAX_COINS_PER_DAY {
            println!("🚫 Sprout is full for today. (Daily Cap Reached)");
            return false;
        }

        // 4. EXECUTE FEEDING
        // For Git, we don't need to pop a task, but we still apply rate limits
        if is_git {
            // Logic for git feeding
        } else {
            // Logic for manual task popping
            if self.tasks.pop().is_none() {
                println!("🚫 No tasks to complete.");
                return false;
            }
        }

        self.coins += 10;
        self.daily_feed_count += 10;
        self.last_fed = now;
        self.save();
        true
    }
}

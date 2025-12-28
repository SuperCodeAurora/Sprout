use rand::Rng;

// 🎭 Mood Definitions
// Sprout 并不是每时每刻都盯着你。有时候它在发呆。
enum Mood {
    Happy,
    Sleepy,
    Hyper,
    Suspicious, // For Cheaters
}

// Helper to pick a random face part
fn get_eyes(mood: Mood) -> &'static str {
    let mut rng = rand::thread_rng();
    match mood {
        Mood::Happy => {
            let options = ["o_o", "^_^", "O_O", "n_n", "¬_¬"];
            options[rng.gen_range(0..options.len())]
        },
        Mood::Sleepy => {
            let options = ["-_-", "u_u", "=.=", "z_z"];
            options[rng.gen_range(0..options.len())]
        },
        Mood::Hyper => {
            let options = ["@_@", "*_*", "Ò_Ó", "x_x"];
            options[rng.gen_range(0..options.len())]
        },
        Mood::Suspicious => {
            let options = ["?_?", "$_$", "x_o", "o_x"]; // Glitched eyes
            options[rng.gen_range(0..options.len())]
        }
    }
}

// 🔥 DYNAMIC ASSET GENERATOR
// Input: Coins, Is_Cheater
pub fn get_plant_art(coins: u64, is_cheater: bool) -> String {
    
    // 1. Check for CHEATER status first
    if is_cheater {
        let eyes = get_eyes(Mood::Suspicious);
        return format!(r#"
      🦠
     [{}]  <-- (CORRUPTED)
    /|   |\
   -- GLITCH --
"#, eyes);
    }

    // 2. Normal Evolution Logic with Dynamic Eyes
    let eyes = get_eyes(Mood::Happy); // Default mood for now

    if coins < 50 {
        // Seedling
        format!(r#"
      🌱
     [ {} ]
    /     \
 -- SPROUT --
"#, eyes)

    } else if coins < 200 {
        // Sapling
        format!(r#"
      🌿
     ({})
    /|   |\
   -- BABY --
"#, eyes)

    } else if coins < 1000 {
        // Scholar (Glasses are fixed, but mouth could change?)
        // keeping static for specific prop consistency
        r#"
      🌳
     (⌐■_■)
    /| 📖 |\
  -- SCHOLAR --
"#.to_string()

    } else if coins < 5000 {
        // Hero
        format!(r#"
       🌲
      /  \
    ⚔️({})🛡️
     /|  |\
    -- HERO --
"#, get_eyes(Mood::Hyper)) // Hero is always intense

    } else if coins < 10000 {
        // Cyber Ent (Matrix vibes)
        format!(r#"
       🤖
     [10101]
    /({}) \
   -- CYBER --
"#, "0_0") // Cyber eyes are fixed

    } else {
        // GOD MODE (Expanded)
        format!(r#"
       👑
     ✨🌟✨
  🪐( {} )🪐
   /  ||  \
  -- COSMIC --
"#, " 👁️ 👄 👁️ ") // Special God Face
    }
}

// Static Utility States
pub const COMA: &str = r#"
      🥀
     (x_x)
    / ... \
   [ SYSTEM FAILURE ]
"#;

pub const FROZEN: &str = r#"
    ❄️❄️❄️
   (🧊-_-🧊)
    ❄️❄️❄️
"#;

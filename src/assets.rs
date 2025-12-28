use rand::Rng;

// 🎭 Mood Definitions
pub enum Mood {
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
            let options = ["?_?", "$_$", "x_o", "o_x"]; 
            options[rng.gen_range(0..options.len())]
        }
    }
}

// 🔥 DYNAMIC ASSET GENERATOR
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

    // 2. Normal Evolution Logic
    let eyes = get_eyes(Mood::Happy); 

    if coins < 50 {
        format!(r#"
      🌱
     [ {} ]
    /     \
 -- SPROUT --
"#, eyes)
    } else if coins < 200 {
        format!(r#"
      🌿
     ({})
    /|   |\
   -- BABY --
"#, eyes)
    } else if coins < 250 {
        r#"
      🌳
     (⌐■_■)
    /| 📖 |\
  -- SCHOLAR --
"#.to_string()
    } else if coins < 1000 {
        format!(r#"
       🌳
      /  \
     ( ˘ ▽ ˘ )
    /|  👔  |\
    -- ADULT --
"#)
    } else if coins < 5000 {
        format!(r#"
       🌲
      /  \
    ⚔️({})🛡️
     /|  |\
    -- HERO --
"#, get_eyes(Mood::Hyper))
    } else if coins < 10000 {
        format!(r#"
       🤖
     [10101]
    /({}) \
   -- CYBER --
"#, "0_0")
    } else {
        format!(r#"
       👑
     ✨🌟✨
  🪐( {} )🪐
   /  ||  \
  -- COSMIC --
"#, " 👁️ 👄 👁️ ")
    }
}

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

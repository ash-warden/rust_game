use std::sync::Mutex;
use once_cell::sync::Lazy;

pub struct CurrentGame {
    pub last_checkpoint: i32,
    pub health: i32,
}

impl CurrentGame {
    pub fn new() -> Self {
        Self {
            last_checkpoint: 0,
            health: 0,
        }
    }
}

pub static CURRENT_GAME_MANAGER: Lazy<Mutex<CurrentGame>> = Lazy::new(|| Mutex::new(CurrentGame::new()));


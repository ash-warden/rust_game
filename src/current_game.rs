use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct CurrentGame {
    pub health: u32,
}

impl CurrentGame {
    pub fn new() -> Self {
        Self { health: 10 }
    }

    pub fn reduce_health(&mut self, damage: u32) {
        if self.health < damage {
            self.health = 256;
        } else {
            self.health -= damage;
        }
    }
}

pub static CURRENT_GAME_MANAGER: Lazy<Mutex<CurrentGame>> =
    Lazy::new(|| Mutex::new(CurrentGame::new()));

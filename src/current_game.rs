use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const MAX_HEALTH: u32 = 10;

pub struct CurrentGame {
    pub health: u32,
}

impl CurrentGame {
    pub fn new() -> Self {
        Self { health: MAX_HEALTH }
    }

    pub fn reduce_health(&mut self, damage: u32) {
        if self.health < damage {
            self.health = MAX_HEALTH;
        } else {
            self.health -= damage;
        }
    }
}

pub static CURRENT_GAME_MANAGER: Lazy<Mutex<CurrentGame>> =
    Lazy::new(|| Mutex::new(CurrentGame::new()));

use once_cell::sync::Lazy;
use std::{collections::HashSet, sync::Mutex};

pub const MAX_HEALTH: u32 = 10;

pub enum GameStages {
    Day1,
    Evening,
    Normal1,
}
pub struct CurrentGame {
    pub health: u32,
    pub stars_collected: HashSet<(String, i32)>,
}

impl CurrentGame {
    pub fn new() -> Self {
        Self {
            health: MAX_HEALTH,
            stars_collected: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.health = MAX_HEALTH;
        self.stars_collected.clear();
    }

    pub fn reduce_health(&mut self, damage: u32) {
        if self.health < damage {
            self.health = MAX_HEALTH;
        } else {
            self.health -= damage;
        }
    }

    pub fn collect_star(&mut self, area: &str, id: i32) {
        self.stars_collected.insert((area.to_string(), id));
    }

    pub fn is_star_collected(&mut self, area: &str, id: i32) -> bool {
        let star_to_find = (area.to_owned(), id);
        self.stars_collected.contains(&star_to_find)
    }

    pub fn get_stars(&self) -> &HashSet<(String, i32)> {
        &self.stars_collected
    }
}

pub static CURRENT_GAME_MANAGER: Lazy<Mutex<CurrentGame>> =
    Lazy::new(|| Mutex::new(CurrentGame::new()));

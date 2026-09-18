use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use crate::item::ItemInv;

pub const MAX_HEALTH: u32 = 10;

pub enum GameStages {
    Day1,
    Evening,
    Normal1,
}
pub struct CurrentGame {
    pub save_name: String,
    pub health: u32,
    pub stars_collected: HashSet<(String, i32)>,
    pub temp_value: i32,
    pub inventory: Vec<ItemInv>,
}

impl CurrentGame {
    pub fn new() -> Self {
        Self {
            health: MAX_HEALTH,
            stars_collected: HashSet::new(),
            save_name: "".to_string(),
            temp_value: 0,
            inventory: vec![],
        }
    }

    pub fn edit_temp(&mut self, new_value: i32) {
        self.temp_value = new_value;
        println!("{}", self.temp_value);
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

    pub fn add_to_inv(&mut self, item: ItemInv) {
        self.inventory.push(item);
    }

    pub fn item_in_inv(&self, item_name: &str) -> i32 {
        let mut count = 0;
        for i in &self.inventory {
            if i.name == item_name {
                count += 1;
            }
        }
        count
    }

    pub fn test_print_inv(&self) {
        for i in &self.inventory {
            println!("{}", i.name);
        }
    }
}

pub static CURRENT_GAME_MANAGER: LazyLock<Mutex<CurrentGame>> =
    LazyLock::new(|| Mutex::new(CurrentGame::new()));

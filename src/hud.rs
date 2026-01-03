use macroquad::{
    color::{Color, RED},
    math::vec2,
};

use crate::{current_game::CURRENT_GAME_MANAGER, text::write_text};

pub struct Hud {}

impl Hud {
    pub fn draw() {
        let health_color = Color::new(1., 0.3, 0.3, 0.5);
        write_text("HEALTH", vec2(0., 0.), health_color);
        let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        let health = cur_game.health;
        write_text(&health.to_string(), vec2(0., 32.), health_color);
    }
}

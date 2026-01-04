use std::cmp::min;

use macroquad::{
    color::Color,
    math::vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use crate::{
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    text::write_text,
};

pub struct Hud {}

impl Hud {
    pub fn draw() {
        //health
        let pos = vec2(16., 16.);
        let thickness = 16.;
        // label
        write_text(
            "HEALTH",
            pos,
            Color::new(1., 0.3, 0.3, 0.5),
            "font_bold.png",
        );
        let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        let health = cur_game.health;
        //meter background
        draw_rectangle_lines(
            pos.x - 1.,
            pos.y + 31.,
            MAX_HEALTH as f32 * 10. + 2.,
            thickness + 2.,
            2.,
            Color::new(0., 0., 0., 0.5),
        );
        draw_rectangle(
            pos.x + 10. * health as f32,
            pos.y + 32.,
            (MAX_HEALTH - min(health, MAX_HEALTH)) as f32 * 10.,
            thickness,
            Color::new(1., 0.3, 0.3, 0.5),
        );
        //meter
        draw_rectangle(
            pos.x,
            pos.y + 32.,
            10. * health as f32,
            thickness,
            Color::new(0.3, 0.3, 1., 0.5),
        );
    }
}

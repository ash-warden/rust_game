use std::cmp::min;

use macroquad::{
    color::Color,
    math::{Vec2, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use crate::{
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    text::write_text,
};

pub struct Hud {}

impl Hud {
    fn draw_health_bar(pos: Vec2, transparency: f32) {
        //health
        let thickness = 16.;
        let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        let health = cur_game.health;
        //meter background
        draw_rectangle_lines(
            pos.x - 1.,
            pos.y - 1.,
            MAX_HEALTH as f32 * 10. + 2.,
            thickness + 2.,
            2.,
            Color::new(0., 0., 0., transparency),
        );
        draw_rectangle(
            pos.x + 10. * health as f32,
            pos.y,
            (MAX_HEALTH - min(health, MAX_HEALTH)) as f32 * 10.,
            thickness,
            Color::new(1., 0.3, 0.3, transparency),
        );
        //meter
        draw_rectangle(
            pos.x,
            pos.y,
            10. * health as f32,
            thickness,
            Color::new(0.3, 0.3, 1., transparency),
        );
    }

    fn draw_health_text(pos: Vec2, transparency: f32) {
        // label
        write_text(
            "HEALTH",
            vec2(pos.x, pos.y + 16.),
            Color::new(1., 0.3, 0.3, transparency),
            "font_bold.png",
        );
    }

    pub fn draw(bottom: bool, solid: bool) {
        let pos = if bottom {
            vec2(24., 324.) // change again once map done
        } else {
            vec2(24., 24.)
        };
        let transparency = if solid { 1.0 } else { 0.3 };
        Self::draw_health_bar(pos, transparency);
        if solid {
            Self::draw_health_text(pos, transparency);
        }
    }
}

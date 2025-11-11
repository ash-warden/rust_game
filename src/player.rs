use crate::resources::RESOURCE_MANAGER;
use crate::{coords_to_index, level};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::{IVec2, Vec2};
use macroquad::prelude::{WHITE, draw_texture};
use std::sync::Arc;

pub struct Player {
    pub position: IVec2,
    pub velocity: IVec2,
    pub size: IVec2,
    pub on_ground: bool,
    level: Arc<level::Level>,
}

impl Player {
    pub fn new(start_pos: (i32, i32), level: Arc<level::Level>) -> Self {
        Player {
            position: IVec2::from(start_pos),
            velocity: IVec2::new(0, 0),
            on_ground: false,
            size: IVec2::new(32, 64),
            level: level.clone(),
        }
    }

    pub fn draw(&self) {
        let res = RESOURCE_MANAGER.lock().unwrap();
        let tex = res.get_texture("player.png");
        draw_texture(tex, self.position.x as f32, self.position.y as f32, WHITE);
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.velocity.x = -200;
        } else if is_key_down(KeyCode::Right) {
            self.velocity.x = 200;
        } else if is_key_down(KeyCode::Up) {
            self.velocity.y = -200;
        } else if is_key_down(KeyCode::Down) {
            self.velocity.y = 200;
        } else {
            self.velocity.x = 0;
            self.velocity.y = 0;
        }

        if is_key_pressed(KeyCode::Space) {
            self.jump();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let tile_size = 32;
        let gravity = 0.; //800.0;

        self.velocity.y += (gravity * delta_time) as i32;


    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = -400;
            self.on_ground = false;
        }
    }

    pub fn set_velocity(&mut self, vx: i32, vy: i32) {
        self.velocity = IVec2::from((vx, vy));
    }

    pub fn get_tile_coords(&self, tile_size: (i32, i32)) -> (i32, i32) {
        (
            (self.position.x / tile_size.0),
            (self.position.y / tile_size.1),
        )
    }
}

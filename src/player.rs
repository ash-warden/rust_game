use crate::resources::RESOURCE_MANAGER;
use crate::{coords_to_index, level};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::{IVec2, Vec2};
use macroquad::prelude::{WHITE, draw_texture};
use std::sync::Arc;

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: IVec2,
    pub on_ground: bool,
    level: Arc<level::Level>,
}

impl Player {
    pub fn new(start_pos: (i32, i32), level: Arc<level::Level>) -> Self {
        Player {
            position: Vec2::new(start_pos.0 as f32, start_pos.1 as f32),
            velocity: Vec2::new(0.0, 0.0),
            on_ground: false,
            size: IVec2::new(32, 64),
            level: level.clone(),
        }
    }

    pub fn draw(&self) {
        let res = RESOURCE_MANAGER.lock().unwrap();
        let tex = res.get_texture("player.png");
        draw_texture(tex, self.position.x, self.position.y, WHITE);
    }

    pub fn handle_input(&mut self, _delta_time: f32) {
        let accel = 1500.0;
        let max_speed = 200.0;
        let friction = 1200.0;

        let mut want_dir: f32 = 0.0;
        if is_key_down(KeyCode::Left) {
            want_dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            want_dir += 1.0;
        }

        if want_dir.abs() > 0.0 {
            self.velocity.x += want_dir * accel * 1.0 / 60.0;
            if self.velocity.x > max_speed { self.velocity.x = max_speed; }
            if self.velocity.x < -max_speed { self.velocity.x = -max_speed; }
        } else {
            if self.velocity.x > 0.0 {
                self.velocity.x -= friction * 1.0 / 60.0;
                if self.velocity.x < 0.0 { self.velocity.x = 0.0; }
            } else if self.velocity.x < 0.0 {
                self.velocity.x += friction * 1.0 / 60.0;
                if self.velocity.x > 0.0 { self.velocity.x = 0.0; }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.jump();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let tile_size = 32.0;
        let gravity = 1800.0;
        let max_fall_speed = 1200.0;
        let epsilon = 0.001;
        let snap_threshold = 3.0; // snap when within 1 pixel

        self.velocity.y += gravity * delta_time;
        if self.velocity.y > max_fall_speed {
            self.velocity.y = max_fall_speed;
        }

        let prev_pos = self.position;

        let move_x = self.velocity.x * delta_time;
        let move_y = self.velocity.y * delta_time;

        self.position.x += move_x;
        self.resolve_axis_collisions(true, prev_pos, tile_size, epsilon, snap_threshold);

        self.position.y += move_y;
        self.on_ground = false;
        self.resolve_axis_collisions(false, prev_pos, tile_size, epsilon, snap_threshold);
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = -600.0;
            self.on_ground = false;
        }
    }

    pub fn set_velocity(&mut self, vx: i32, vy: i32) {
        self.velocity = Vec2::new(vx as f32, vy as f32);
    }

    pub fn get_tile_coords(&self, tile_size: (i32, i32)) -> (i32, i32) {
        (
            (self.position.x as i32 / tile_size.0),
            (self.position.y as i32 / tile_size.1),
        )
    }

    fn resolve_axis_collisions(&mut self, axis_x: bool, prev_pos: Vec2, tile_size: f32, epsilon: f32, snap_threshold: f32) {
        let left = self.position.x;
        let top = self.position.y;
        let right = self.position.x + self.size.x as f32;
        let bottom = self.position.y + self.size.y as f32;

        let tile_left = (left / tile_size).floor() as i32;
        let tile_right = ((right - epsilon) / tile_size).floor() as i32;
        let tile_top = (top / tile_size).floor() as i32;
        let tile_bottom = ((bottom - epsilon) / tile_size).floor() as i32;

        let dir_x = (self.position.x - prev_pos.x).signum();
        let dir_y = (self.position.y - prev_pos.y).signum();

        for ty in tile_top..=tile_bottom {
            for tx in tile_left..=tile_right {
                if !self.is_tile_solid(tx, ty) { continue; }

                let tile_px_left = tx as f32 * tile_size;
                let tile_px_top = ty as f32 * tile_size;
                let tile_px_right = tile_px_left + tile_size;
                let tile_px_bottom = tile_px_top + tile_size;

                if axis_x {
                    let overlap_left = right - tile_px_left;
                    let overlap_right = tile_px_right - left;

                    if overlap_left > 0.0 && overlap_right > 0.0 {
                        if overlap_left < overlap_right {
                            // overlap from left side
                            if overlap_left <= snap_threshold {
                                // snap to exact border
                                self.position.x = tile_px_left - self.size.x as f32;
                            } else {
                                self.position.x -= overlap_left;
                            }
                            self.velocity.x = 0.0;
                        } else {
                            // overlap from right side
                            if overlap_right <= snap_threshold {
                                self.position.x = tile_px_right;
                            } else {
                                self.position.x += overlap_right;
                            }
                            self.velocity.x = 0.0;
                        }
                    }
                } else {
                    let overlap_top = bottom - tile_px_top;
                    let overlap_bottom = tile_px_bottom - top;

                    if overlap_top > 0.0 && overlap_bottom > 0.0 {
                        if overlap_top < overlap_bottom {
                            // landed on tile
                            if overlap_top <= snap_threshold {
                                self.position.y = tile_px_top - self.size.y as f32;
                            } else {
                                self.position.y -= overlap_top;
                            }
                            self.velocity.y = 0.0;
                            self.on_ground = true;
                        } else {
                            // hit head
                            if overlap_bottom <= snap_threshold {
                                self.position.y = tile_px_bottom;
                            } else {
                                self.position.y += overlap_bottom;
                            }
                            self.velocity.y = 0.0;
                        }
                    }
                }
            }
        }

        // final safety: if we're within snap_threshold of a tile grid line vertically and not overlapping a tile,
        // optionally snap horizontal/vertical alignment to avoid accumulating subpixel error.
        // Example: snap x to nearest tile column if close (optional)
        // let col = (self.position.x / tile_size).round();
        // if (self.position.x - col * tile_size).abs() <= snap_threshold { self.position.x = col * tile_size; }
    }
    // Return true if tile at (tx, ty) is solid (collidable).
    fn is_tile_solid(&self, tx: i32, ty: i32) -> bool {
        // Example placeholder:
        // If you have something like self.level.is_solid(tx, ty) use that.
        // Here we assume out-of-bounds is solid to prevent leaving level.
        if tx < 0 || ty < 0 {
            return true;
        }
        if tx > 20 || ty > 15 {
            return true;
        }
        // Replace the following line with your level's tile check:
        return self.level.get_tile_info((tx, ty)).solid;
    }
}
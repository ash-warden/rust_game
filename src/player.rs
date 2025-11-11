use crate::level;
use crate::resources::RESOURCE_MANAGER;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::{IVec2, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, WHITE, draw_texture_ex};
use std::sync::Arc;

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub actual_size: IVec2,
    pub full_size: i32,
    pub on_ground: bool,
    level: Arc<level::Level>,
    state: PlayerState,
    facing_right: bool,
}

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Standing,
    Walking,
    Running,
    Jumping,
    Falling,
    Crouching,
}

impl Player {
    pub fn new(start_pos: (i32, i32), level: Arc<level::Level>) -> Self {
        Player {
            position: Vec2::new(start_pos.0 as f32, start_pos.1 as f32),
            velocity: Vec2::new(0.0, 0.0),
            on_ground: false,
            actual_size: IVec2::new(32, 64),
            level: level.clone(),
            state: PlayerState::Standing,
            facing_right: true,
            full_size: 64,
        }
    }

    pub fn draw(&self) {
        let res = RESOURCE_MANAGER.lock().unwrap();
        let tex = res.get_texture("player.png");
        draw_texture_ex(
            tex,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                flip_x: !self.facing_right,
                dest_size: Some(vec2(self.actual_size.x as f32, self.actual_size.y as f32)),
                ..Default::default()
            },
        );
    }

    pub fn handle_input(&mut self, _delta_time: f32) {
        let accel = 1500.0;
        let max_speed = 200.0;
        let friction = 1200.0;

        let mut want_dir: f32 = 0.0;
        if is_key_down(KeyCode::Left) {
            self.facing_right = false;
            want_dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            self.facing_right = true;
            want_dir += 1.0;
        }

        if want_dir.abs() > 0.0 {
            self.velocity.x += want_dir * accel * 1.0 / 60.0;
            //limit max speed
            if self.velocity.x > max_speed {
                self.velocity.x = max_speed;
            }
            if self.velocity.x < -max_speed {
                self.velocity.x = -max_speed;
            }
        } else {
            //friction
            if self.velocity.x > 0.0 {
                self.velocity.x -= friction * 1.0 / 60.0;
                if self.velocity.x < 0.0 {
                    self.velocity.x = 0.0;
                }
            } else if self.velocity.x < 0.0 {
                self.velocity.x += friction * 1.0 / 60.0;
                if self.velocity.x > 0.0 {
                    self.velocity.x = 0.0;
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.jump();
        }
        if is_key_pressed(KeyCode::Down) {
            self.crouch();
        }
        if is_key_pressed(KeyCode::Up) {
            self.uncrouch();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // setting state for movement
        if !(self.state == PlayerState::Crouching) {
            if self.on_ground {
                if (self.velocity.x.abs()) > 199. {
                    self.state = PlayerState::Running;
                } else if self.velocity.x.abs() > 0. {
                    self.state = PlayerState::Walking;
                } else {
                    self.state = PlayerState::Standing;
                }
            } else {
                if self.velocity.y > 0. {
                    self.state = PlayerState::Falling;
                } else {
                    self.state = PlayerState::Jumping;
                }
            }
        }

        println!("{:?}", self.state);
        let tile_size = self.level.tile_size;
        let gravity = 1800.0;
        let max_fall_speed = 1200.0;
        let epsilon = 0.001;
        let snap_threshold = 3.0; // snap when within this many pixels

        // apply gravity
        self.velocity.y += gravity * delta_time;
        if self.velocity.y > max_fall_speed {
            self.velocity.y = max_fall_speed;
        }

        // horizontal movement handled once per frame
        let move_x = self.velocity.x * delta_time;
        self.position.x += move_x;
        // use prev_pos from before any sub-steps for horizontal resolution (keeps X resolution consistent)
        let prev_frame_pos = self.position;
        self.resolve_axis_collisions(true, prev_frame_pos, tile_size, epsilon, snap_threshold);

        // vertical movement: sub-step to avoid tunnelling/bounce
        let total_move_y = self.velocity.y * delta_time;
        // maximum pixels per sub-step (tune down if bounce still happens)
        let max_step = 1.0_f32;
        let mut remaining = total_move_y;
        // we will iterate sub-steps; prev_pos should be updated each sub-step for reliable "came_from_above" checks
        let mut step_prev_pos = self.position;

        while remaining.abs() > 0.0 {
            let step = if remaining.abs() > max_step {
                max_step * remaining.signum()
            } else {
                remaining
            };
            self.position.y += step;
            // reset on_ground before resolving this sub-step; it will be set true if this sub-step lands
            self.on_ground = false;
            self.resolve_axis_collisions(false, step_prev_pos, tile_size, epsilon, snap_threshold);

            // after resolution, if we landed, zero vertical velocity and clear remaining (we shouldn't continue moving down)
            if self.on_ground && self.velocity.y > 0.0 {
                self.velocity.y = 0.0;
                break;
            }

            // subtract processed step and update prev for next sub-step
            remaining -= step;
            step_prev_pos = self.position;
            // safety: break loop if something goes wrong
            // (prevents infinite loop with NaNs)
            if !remaining.is_finite() {
                break;
            }
        }
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = -650.0;
            self.on_ground = false;
        }
    }

    pub fn crouch(&mut self) {
        if self.state != PlayerState::Crouching {
            self.actual_size.y = self.full_size  / 2;
            self.state = PlayerState::Crouching;
            self.position.y += self.full_size as f32 / 2.;
        }
    }

    fn can_uncrouch(&self, tile_size: f32, epsilon: f32) -> bool {
        let head_clearance = self.full_size as f32 / 2.;
        let check_top = self.position.y - head_clearance;
        let left = self.position.x;
        let right = self.position.x + self.actual_size.x as f32;

        let tile_left = (left / tile_size).floor() as i32;
        let tile_right = ((right - epsilon) / tile_size).floor() as i32;
        let tile_check_y = (check_top / tile_size).floor() as i32;

        for tx in tile_left..=tile_right {
            if self.is_tile_solid(tx, tile_check_y) {
                return false; // blocked by tile above
            }
        }

        true // space is clear to uncrouch
    }

    pub fn uncrouch(&mut self) {
        if self.can_uncrouch(self.level.tile_size, 0.001) && self.state == PlayerState::Crouching {
            self.actual_size.y = self.full_size;
            self.state = PlayerState::Jumping;
            self.position.y -= (self.full_size / 2) as f32;
        }
    }

    fn resolve_axis_collisions(
        &mut self,
        axis_x: bool,
        prev_pos: Vec2,
        tile_size: f32,
        epsilon: f32,
        snap_threshold: f32,
    ) {
        let left = self.position.x;
        let top = self.position.y;
        let right = self.position.x + self.actual_size.x as f32;
        let bottom = self.position.y + self.actual_size.y as f32;

        let tile_left = (left / tile_size).floor() as i32;
        let tile_right = ((right - epsilon) / tile_size).floor() as i32;
        let tile_top = (top / tile_size).floor() as i32;
        let tile_bottom = ((bottom - epsilon) / tile_size).floor() as i32;

        for ty in tile_top..=tile_bottom {
            for tx in tile_left..=tile_right {
                if !self.is_tile_solid(tx, ty) {
                    continue;
                }

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
                                self.position.x = tile_px_left - self.actual_size.x as f32;
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
                    // vertical resolution improved: use prev_pos to detect genuine landings / head-hits
                    let prev_bottom = prev_pos.y + self.actual_size.y as f32;
                    let prev_top = prev_pos.y;

                    let overlap_top = bottom - tile_px_top; // positive if overlapping from above
                    let overlap_bottom = tile_px_bottom - top; // positive if overlapping from below

                    if overlap_top > 0.0 && overlap_bottom > 0.0 {
                        // Determine whether this collision should be treated as landing or head hit,
                        // using previous position to see where we came from.
                        let came_from_above = prev_bottom <= tile_px_top + epsilon;
                        let came_from_below = prev_top >= tile_px_bottom - epsilon;

                        if came_from_above {
                            // landing on tile
                            if overlap_top <= snap_threshold {
                                self.position.y = tile_px_top - self.actual_size.y as f32;
                            } else {
                                self.position.y -= overlap_top;
                            }
                            self.velocity.y = 0.0;
                            self.on_ground = true;
                        } else if came_from_below {
                            // hit head
                            if overlap_bottom <= snap_threshold {
                                self.position.y = tile_px_bottom;
                            } else {
                                self.position.y += overlap_bottom;
                            }
                            self.velocity.y = 0.0;
                        } else {
                            // ambiguous (we were already overlapping or a large tunnelling move); pick smallest
                            if overlap_top < overlap_bottom {
                                if overlap_top <= snap_threshold {
                                    self.position.y = tile_px_top - self.actual_size.y as f32;
                                } else {
                                    self.position.y -= overlap_top;
                                }
                                self.velocity.y = 0.0;
                                self.on_ground = true;
                            } else {
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
        }
    }

    // Return true if tile at (tx, ty) is solid (collidable).
    fn is_tile_solid(&self, tx: i32, ty: i32) -> bool {
        if tx < 0 || ty < 0 {
            return true;
        }
        if tx > 20 || ty > 15 {
            return true;
        }
        return self.level.get_tile_info((tx, ty)).solid;
    }
}

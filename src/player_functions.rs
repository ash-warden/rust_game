use crate::controls::CONTROLS;
use crate::player::{
    PLAYER_ACCEL, PLAYER_EPSILON, PLAYER_FRICTION, PLAYER_GRAVITY, PLAYER_JUMP_MIN,
    PLAYER_JUMP_VARY_LIMIT, PLAYER_MAX_FALL_SPEED, PLAYER_SNAP_THRESHOLD, PLAYER_SPEED_RUN,
    PLAYER_SPEED_WALK, Player,
};
use macroquad::math::{IVec2, Vec2, ivec2, vec2};

impl Player {
    pub fn calc_horizontal_velocity(&mut self, delta_time: f32) {
        let max_speed: f32;
        let mut want_dir: f32 = 0.0;
        let mut input = CONTROLS.lock().unwrap();
        if input.controls_left() {
            self.facing_right = false;
            want_dir -= 1.0;
        }
        if input.controls_right() {
            self.facing_right = true;
            want_dir += 1.0;
        }
        if input.controls_secondary() {
            max_speed = PLAYER_SPEED_RUN;
        } else {
            max_speed = PLAYER_SPEED_WALK;
        }

        // Desired velocity based on input
        let target_velocity = want_dir * max_speed;

        // Difference between current and desired
        let delta = target_velocity - self.velocity.x;

        if delta.abs() > 0.0 {
            // If we need to slow down (opposite direction or stopping), use friction
            let rate = if target_velocity.signum() != self.velocity.x.signum() {
                PLAYER_FRICTION
            } else {
                PLAYER_ACCEL
            };

            // Move velocity toward target
            let step = rate * delta_time;
            if delta.abs() <= step {
                self.velocity.x = target_velocity;
            } else {
                self.velocity.x += delta.signum() * step;
            }
        }
    }

    pub fn handle_jumping(&mut self) {
        let mut input = CONTROLS.lock().unwrap();
        if input.controls_primary() {
            self.jump();
        }

        //variable jump height
        if !input.controls_primary() && self.velocity.y < PLAYER_JUMP_VARY_LIMIT {
            self.velocity.y = self.velocity.y / 2.;
        }
    }

    pub fn apply_gravity(&mut self, delta_time: f32) {
        self.velocity.y += PLAYER_GRAVITY * delta_time;
        if self.velocity.y > PLAYER_MAX_FALL_SPEED {
            self.velocity.y = PLAYER_MAX_FALL_SPEED;
        }
    }

    pub fn horizontal_movement(&mut self, delta_time: f32) {
        let tile_size = self.level.tile_size;
        let move_x = self.velocity.x * delta_time;
        self.position.x += move_x;
        // use prev_pos from before any sub-steps for horizontal resolution (keeps X resolution consistent)
        let prev_frame_pos = self.position;
        self.resolve_axis_collisions(
            true,
            prev_frame_pos,
            tile_size as f32,
            PLAYER_EPSILON,
            PLAYER_SNAP_THRESHOLD,
        );
    }

    pub fn vertical_movement(&mut self, delta_time: f32) {
        let tile_size = self.level.tile_size;
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
            self.resolve_axis_collisions(
                false,
                step_prev_pos,
                tile_size as f32,
                PLAYER_EPSILON,
                PLAYER_SNAP_THRESHOLD,
            );

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
            println!("{}", self.velocity.x);
            self.velocity.y = -self.velocity.x.abs() / 3. - PLAYER_JUMP_MIN;
            self.on_ground = false;
        }
    }

    pub fn resolve_axis_collisions(
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

    pub fn check_for_ladder(&self) -> Option<IVec2> {
        let offsets1 = [
            //vec2(0., 0.),
            vec2(0., self.actual_size.y as f32 / 2.),
            //vec2(0., self.actual_size.y as f32 - 1.),
            //vec2(self.actual_size.x as f32 - 1., 0.),
            vec2(
                self.actual_size.x as f32 - 1.,
                self.actual_size.y as f32 / 2.,
            ),
            //vec2(self.actual_size.x as f32 - 1., self.actual_size.y as f32 - 1.),
        ];
        let offsets2 = [
            //vec2(0., 0.),
            //vec2(0., self.actual_size.y as f32 / 2.),
            vec2(0., self.actual_size.y as f32 - 1.),
            //vec2(self.actual_size.x as f32 - 1., 0.),
            /*vec2(
                self.actual_size.x as f32 - 1.,
                self.actual_size.y as f32 / 2.,
            ),*/
            vec2(
                self.actual_size.x as f32 - 1.,
                self.actual_size.y as f32 - 1.,
            ),
        ];
        let mut coords: IVec2 = ivec2(0, 0);
        let on_ladder = offsets1.iter().any(|offset1| {
            coords = ((self.position + *offset1).as_ivec2()) / 32;
            self.level.get_tile_info(coords).ladder
        }) && offsets2.iter().any(|offset2| {
            coords = ((self.position + *offset2).as_ivec2()) / 32;
            self.level.get_tile_info(coords).ladder
        });
        if on_ladder {
            Option::from(coords)
        } else {
            None
        }
    }

    pub fn over_solid_tile(&self) -> bool {
        let offsets = [
            vec2(0., 0.),
            vec2(0., self.actual_size.y as f32 / 2.),
            vec2(0., self.actual_size.y as f32 - 1.),
            vec2(self.actual_size.x as f32 - 1., 0.),
            vec2(
                self.actual_size.x as f32 - 1.,
                self.actual_size.y as f32 / 2.,
            ),
            vec2(
                self.actual_size.x as f32 - 1.,
                self.actual_size.y as f32 - 1.,
            ),
        ];
        offsets.iter().any(|offset| {
            let coords = ((self.position + *offset).as_ivec2()) / 32;
            self.level.get_tile_info(coords).solid
        })
    }

    pub fn want_to_climb() -> bool {
        let mut input = CONTROLS.lock().unwrap();
        (input.controls_up() || input.controls_down())
            && !(input.controls_left() || input.controls_right())
    }
    //return true to get off ladder
    pub fn handle_climb(&mut self, col: i32) -> bool {
        let ladder_pos_x = (col * self.level.tile_size + self.level.tile_size / 2) as f32;
        let player_centre = self.position.x + (self.actual_size.x / 2) as f32;
        if player_centre - ladder_pos_x > 5.1 {
            self.position.x -= 5.;
        } else if player_centre - ladder_pos_x < -5.1 {
            self.position.x += 5.;
        } else {
            self.position.x = ladder_pos_x - (self.actual_size.x / 2) as f32;
        }
        let mut input = CONTROLS.lock().unwrap();
        self.velocity = vec2(0., 0.);
        if (input.controls_left() || input.controls_right() || input.controls_primary()) && !self.over_solid_tile() {
            return true;
        } else if input.controls_up() {
            self.position.y -= 2.;
            if self.check_for_ladder() == None {
                self.position.y += 2.;
            }
        } else if input.controls_down() {
            self.position.y += 2.;
            if self.check_for_ladder() == None {
                self.position.y -= 2.;
            }
        }
        false
    }

    // Return true if tile at (tx, ty) is solid.
    fn is_tile_solid(&self, tx: i32, ty: i32) -> bool {
        self.level.get_tile_info(ivec2(tx, ty)).solid
    }
}

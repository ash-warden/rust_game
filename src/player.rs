use crate::current_game::CURRENT_GAME_MANAGER;
use crate::level;
use crate::player_functions::Direction;
use crate::resources::RESOURCE_MANAGER;
use macroquad::math::{IVec2, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, WHITE, draw_texture_ex};
use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub actual_size: IVec2,
    pub on_ground: bool,
    pub level: Arc<level::Room>,
    pub state: PlayerMovementState,
    pub facing_right: bool,
    pub damage_timer: f32,
}

pub struct PlayerInitialInfo {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub state: PlayerMovementState,
}

pub const PLAYER_ACCEL: f32 = 600.0;
pub const PLAYER_SPEED_WALK: f32 = 200.0;
pub const PLAYER_SPEED_RUN: f32 = PLAYER_SPEED_WALK * 1.7;
pub const PLAYER_FRICTION: f32 = 1000.;
pub const PLAYER_GRAVITY: f32 = 1600.;
pub const PLAYER_MAX_FALL_SPEED: f32 = 600.0;
pub const PLAYER_EPSILON: f32 = 0.001;
pub const PLAYER_SNAP_THRESHOLD: f32 = 3.0;
pub const PLAYER_JUMP_MIN: f32 = 600.;
pub const PLAYER_JUMP_VARY_LIMIT: f32 = -200.;

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerMovementState {
    Standing,
    Walking,
    Running,
    Jumping,
    Falling,
    Climbing,
}

impl Player {
    pub fn new(
        start_pos: Vec2,
        level: Arc<level::Room>,
        velocity: Vec2,
        state: PlayerMovementState,
    ) -> Self {
        Player {
            position: Vec2::new(start_pos.x, start_pos.y),
            velocity,
            on_ground: false,
            actual_size: IVec2::new(24, 64),
            level: level.clone(),
            state,
            facing_right: true,
            damage_timer: 0.,
        }
    }

    pub fn new_from_info(player_info: PlayerInitialInfo, level: Arc<level::Room>) -> Self {
        Player::new(
            player_info.pos,
            level,
            player_info.velocity,
            player_info.state,
        )
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

    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            PlayerMovementState::Standing
            | PlayerMovementState::Walking
            | PlayerMovementState::Running
            | PlayerMovementState::Jumping
            | PlayerMovementState::Falling => {
                if !self.on_ground {
                    if self.velocity.y > 0. {
                        self.state = PlayerMovementState::Falling;
                    } else if self.velocity.y < 0. {
                        self.state = PlayerMovementState::Jumping;
                    }
                } else {
                    self.state = PlayerMovementState::Standing;
                }
                self.calc_horizontal_velocity(delta_time);
                self.handle_jumping();
                self.apply_gravity(delta_time);
                self.horizontal_movement(delta_time);
                self.vertical_movement(delta_time);
                if self.on_ground {
                    let ladder = self.check_for_ladder();
                    if ladder != None {
                        if Self::want_to_climb() {
                            self.state = PlayerMovementState::Climbing;
                        }
                    }
                }
            }
            PlayerMovementState::Climbing => {
                let ladder = self.check_for_ladder();
                if ladder != None {
                    //true is player is trying to get off ladder
                    if self.handle_climb_and_check_done(ladder.unwrap().x) {
                        self.state = PlayerMovementState::Standing;
                    }
                }
            }
        }
        // handle different "horizontal" states
        match self.state {
            PlayerMovementState::Standing
            | PlayerMovementState::Walking
            | PlayerMovementState::Running => {
                if self.velocity.x.abs() > PLAYER_SPEED_WALK {
                    self.state = PlayerMovementState::Running;
                } else if self.velocity.x.abs() > 0.1 {
                    self.state = PlayerMovementState::Walking;
                } else if self.velocity.x.abs() <= 0.1 {
                    self.state = PlayerMovementState::Standing;
                }
            }
            _ => {} //don't do anything when in other states
        }
        if self.on_ground {
            // check what type of tile landed on
            // println!("{:?}", self.touching_hazard(Direction::Below));
            if self.damage_timer <= 0. && self.touching_hazard(Direction::Below) {
                let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
                cur_game.reduce_health(1);
                self.damage_timer = 1.;
            }
            self.damage_timer -= delta_time;
        } else {
            self.damage_timer = 0.;
        }
        // println!("{:?}", self.state);
    }
}

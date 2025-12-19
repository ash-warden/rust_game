use crate::level;
use crate::resources::RESOURCE_MANAGER;
use macroquad::math::{IVec2, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, WHITE, draw_texture_ex};
use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub actual_size: IVec2,
    pub full_size: i32,
    pub on_ground: bool,
    pub level: Arc<level::Level>,
    pub state: PlayerMovementState,
    pub facing_right: bool,
    pub crouching: bool,
}

pub struct PlayerInitialInfo {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub state: PlayerMovementState,
    pub crouch: bool,
}

pub const PLAYER_ACCEL: f32 = 300.0;
pub const PLAYER_SPEED_WALK: f32 = 200.0;
pub const PLAYER_SPEED_RUN: f32 = PLAYER_SPEED_WALK * 1.7;
pub const PLAYER_SPEED_CRAWL: f32 = PLAYER_SPEED_WALK * 0.5;
pub const PLAYER_FRICTION: f32 = 700.;
pub const PLAYER_GRAVITY: f32 = 1600.;
pub const PLAYER_MAX_FALL_SPEED: f32 = 600.0;
pub const PLAYER_EPSILON: f32 = 0.001;
pub const PLAYER_SNAP_THRESHOLD: f32 = 3.0;
pub const PLAYER_JUMP_MIN: f32 = 550.;
pub const PLAYER_JUMP_VARY_LIMIT: f32 = -200.;

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerMovementState {
    Standing,
    Walking,
    Running,
    Jumping,
    Falling,
    Crouching,
    Crawling,
    Climbing,
}

impl Player {
    pub fn new(
        start_pos: Vec2,
        level: Arc<level::Level>,
        velocity: Vec2,
        state: PlayerMovementState,
        crouching: bool,
    ) -> Self {
        Player {
            position: Vec2::new(start_pos.x, start_pos.y),
            velocity,
            on_ground: false,
            actual_size: IVec2::new(24, 64),
            level: level.clone(),
            state,
            facing_right: true,
            full_size: 64,
            crouching,
        }
    }

    pub fn new_from_info(player_info: PlayerInitialInfo, level: Arc<level::Level>) -> Self {
        Player::new(
            player_info.pos,
            level,
            player_info.velocity,
            player_info.state,
            player_info.crouch,
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
        self.calc_horizontal_velocity(delta_time);
        self.handle_jumping();
        //update states
        if self.on_ground {
            if (self.state == PlayerMovementState::Crouching
                || self.state == PlayerMovementState::Crawling)
            {
                if self.velocity.x.abs() < 0.1 {
                    self.state = PlayerMovementState::Crouching;
                    self.velocity.x = 0.;
                } else {
                    self.state = PlayerMovementState::Crawling;
                }
            } else {
                if self.velocity.x.abs() < 0.1 {
                    self.state = PlayerMovementState::Standing;
                    self.velocity.x = 0.;
                } else if self.velocity.x.abs() < 201.0 {
                    self.state = PlayerMovementState::Walking;
                } else {
                    self.state = PlayerMovementState::Running;
                }
            }
        } else {
            if self.state == PlayerMovementState::Climbing {
                //nothing yet
            } else if !(self.state == PlayerMovementState::Crouching
                || self.state == PlayerMovementState::Crawling)
            {
                if self.velocity.y > 0. {
                    self.state = PlayerMovementState::Falling;
                } else {
                    self.state = PlayerMovementState::Jumping;
                }
            }
        }

        //handle crouching
        self.handle_crouching();
        println!("{:?}", self.state);
        self.apply_gravity(delta_time);
        self.horizontal_movement(delta_time);
        self.vertical_movement(delta_time);
    }
}

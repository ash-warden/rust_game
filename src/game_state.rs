use crate::player::{Player, PlayerState};
use crate::resources::RESOURCE_MANAGER;
use crate::{index_to_coords, level};
use macroquad::color::WHITE;
use macroquad::math::{vec2, IVec2, Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams};
use std::sync::Arc;

pub enum StateTransition {
    None,
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
}

pub trait GameState {
    fn update(&mut self) -> StateTransition;
    fn draw(&self, scale: f32);
}

pub struct LevelState {
    pub level: Arc<level::Level>,
    pub player: Player,
}

impl LevelState {
    pub fn build(level: &str, player_pos: Vec2, player_velocity: Vec2, player_state: PlayerState) -> Result<LevelState, Box<dyn std::error::Error>> {
        let res = RESOURCE_MANAGER.lock().unwrap();
        if let Some(level) = res.get_level(level) {
            let player = Player::new(player_pos, level.clone(), player_velocity, player_state);
            Ok(LevelState { level, player })
        } else {
            Err("Level not found in resources".into())
        }
    }
}

#[derive(PartialEq)]
enum DirectionToMove {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl GameState for LevelState {
    fn update(&mut self) -> StateTransition {
        let frame_time = get_frame_time();
        self.player.handle_input(frame_time);
        self.player.update(frame_time);

        // Determine direction based on player position
        let direction = if self.player.position.x < -25.0 {
            DirectionToMove::Left
        } else if self.player.position.x > 640.0 {
            DirectionToMove::Right
        } else if self.player.position.y < -57.0 {
            DirectionToMove::Up
        } else if self.player.position.y > 480.0 {
            DirectionToMove::Down
        } else {
            DirectionToMove::None
        };

        // If no transition, return early
        if direction == DirectionToMove::None {
            return StateTransition::None;
        }

        // Compute new player position and level offset in one match
        let (new_player_pos, offset) = match direction {
            DirectionToMove::Left => (vec2(630.0, self.player.position.y), IVec2::new(-1, 0)),
            DirectionToMove::Right => (vec2(-22.0, self.player.position.y - 1.), IVec2::new(1, 0)),
            DirectionToMove::Up => (vec2(self.player.position.x, 480.0), IVec2::new(0, 1)),
            DirectionToMove::Down => (vec2(self.player.position.x, -25.0), IVec2::new(0, -1)),
            DirectionToMove::None => unreachable!(),
        };

        let new_level = format!(
            "test_{}_{}",
            self.level.x_coord + offset.x,
            self.level.y_coord + offset.y
        );

        // Load new LevelState
        match LevelState::build(&new_level, new_player_pos, self.player.velocity, self.player.state.clone()) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state{}: {err}", &new_level);
                std::process::exit(1);
            }
        }
    }

    fn draw(&self, scale: f32) {
        //draw tiles
        let mut i = 0; //tile number
        let mut x = 0.; //x coord
        let mut y = 0.; //y coord

        let level = &self.level;

        let map_width = level.map_dimensions.0;
        let map_height = level.map_dimensions.1;

        while y < map_height {
            //column
            while x < map_width {
                //row
                let res = RESOURCE_MANAGER.lock().unwrap();
                let tex = res.get_texture(&level.tile_image_name);
                draw_texture_ex(
                    tex,
                    x * level.tile_size * scale,
                    y * level.tile_size * scale,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(level.tile_size * scale, level.tile_size * scale)),
                        source: Some(Rect::new(
                            index_to_coords(level.tile_values[i], level.tileset_columns).0
                                * level.tile_size,
                            index_to_coords(level.tile_values[i], level.tileset_columns).1
                                * level.tile_size,
                            level.tile_size,
                            level.tile_size,
                        )),
                        ..Default::default()
                    },
                );
                x += 1.;
                if i >= level.tile_values.len() {
                    println!("too many tiles to draw!");
                    break;
                }
                i += 1;
            }
            x = 0.; // go back to beginning of row
            y += 1.;
        }
        //draw player
        self.player.draw(scale);
    }
}

pub struct GameStateStack {
    pub states: Vec<Box<dyn GameState>>,
}

impl GameStateStack {
    pub fn new(initial: Box<dyn GameState>) -> Self {
        Self {
            states: vec![initial],
        }
    }

    pub fn update(&mut self) {
        if let Some(state) = self.states.last_mut() {
        match state.update() {
            StateTransition::None => {}
            StateTransition::Replace(new_state) => self.replace(new_state),
            StateTransition::Push(new_state) => self.push(new_state),
            StateTransition::Pop => { self.pop(); }
        }
    }
    }

    pub fn draw(&self, scale: f32) {
        if let Some(state) = self.states.last() {
            state.draw(scale);
        }
    }

    pub fn push(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn replace(&mut self, state: Box<dyn GameState>) {
        self.pop();
        self.push(state);
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

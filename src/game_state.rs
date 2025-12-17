pub(crate) use crate::player::{Player, PlayerInfo};
use crate::resources::RESOURCE_MANAGER;
use crate::{index_to_coords, level};
use macroquad::color::WHITE;
use macroquad::math::{IVec2, Rect, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex, get_frame_time};
use std::sync::Arc;
use crate::menu::Menu;

pub enum StateTransition {
    None,
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
}

pub trait GameState {
    fn update(&mut self) -> StateTransition;
    fn draw(&self);
}

#[derive(Clone)]
pub struct LevelState {
    pub level: Arc<level::Level>,
    pub player: Player,
}

impl LevelState {
    pub fn build(
        level: &str,
        player_info: PlayerInfo,
    ) -> Result<LevelState, Box<dyn std::error::Error>> {
        let res = RESOURCE_MANAGER.lock()?;
        if let Some(level) = res.get_level(level) {
            let player = Player::new_from_info(player_info, level.clone());
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

        let map_width = (self.level.map_dimensions.x * self.level.tile_size) as f32;
        let map_height = (self.level.map_dimensions.y * self.level.tile_size) as f32;

        let p_size_x = self.player.actual_size.x;
        let p_size_y = self.player.actual_size.y;

        let direction = if self.player.position.x < -p_size_x as f32 / 2. {
            DirectionToMove::Left
        } else if self.player.position.x > map_width - p_size_x as f32 / 2. {
            DirectionToMove::Right
        } else if self.player.position.y < -p_size_y as f32 / 2. {
            DirectionToMove::Up
        } else if self.player.position.y > map_height - p_size_y as f32 / 2. {
            DirectionToMove::Down
        } else {
            DirectionToMove::None
        };

        if direction == DirectionToMove::None {
            return StateTransition::None;
        }

        let (new_player_pos, offset) = match direction {
            DirectionToMove::Left => (
                vec2(
                    map_width - p_size_x as f32 / 2. - 1.,
                    self.player.position.y,
                ),
                IVec2::new(-1, 0),
            ),
            DirectionToMove::Right => (
                vec2(-p_size_x as f32 / 2. + 1., self.player.position.y - 1.),
                IVec2::new(1, 0),
            ),
            DirectionToMove::Up => (
                vec2(
                    self.player.position.x,
                    map_height - p_size_y as f32 / 2. - 1.,
                ),
                IVec2::new(0, 1),
            ),
            DirectionToMove::Down => (
                vec2(self.player.position.x, -p_size_y as f32 / 2. + 1.),
                IVec2::new(0, -1),
            ),
            DirectionToMove::None => unreachable!(),
        };

        let new_level = format!(
            "test_{}_{}",
            self.level.x_coord + offset.x,
            self.level.y_coord + offset.y
        );

        let player_info = PlayerInfo {
            pos: new_player_pos,
            velocity: self.player.velocity,
            state: self.player.state.clone(),
            crouch: self.player.crouching,
        };

        // Load new LevelState
        match LevelState::build(&new_level, player_info) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state{}: {err}", &new_level);
                std::process::exit(1);
            }
        }
    }

    fn draw(&self) {
        //draw tiles
        let mut i = 0; //tile number
        let mut x = 0; //x coord
        let mut y = 0; //y coord

        let level = &self.level;

        let map_width = level.map_dimensions.x;
        let map_height = level.map_dimensions.y;

        let t_size = level.tile_size as f32;

        while y < map_height {
            //column
            while x < map_width {
                //row
                let res = RESOURCE_MANAGER.lock().unwrap();
                let tex = res.get_texture(&level.tile_image_name);
                draw_texture_ex(
                    tex,
                    x as f32 * t_size,
                    y as f32 * t_size,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(t_size, t_size)),
                        source: Some(Rect::new(
                            index_to_coords(level.tile_values[i], level.tileset_columns).0 * t_size,
                            index_to_coords(level.tile_values[i], level.tileset_columns).1 * t_size,
                            t_size,
                            t_size,
                        )),
                        ..Default::default()
                    },
                );
                x += 1;
                if i >= level.tile_values.len() {
                    println!("too many tiles to draw!");
                    break;
                }
                i += 1;
            }
            x = 0; // go back to beginning of row
            y += 1;
        }
        //draw player
        self.player.draw();
    }
}

pub struct MenuState {
    pub menu: Menu,
}

impl MenuState {
    pub fn new(menu: Menu) -> Self {
        MenuState{menu}
    }
}

impl GameState for MenuState {
    fn update(&mut self) -> StateTransition {
        self.menu.update()
    }
    fn draw(&self) {
        self.menu.draw();
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
                StateTransition::Pop => {
                    self.pop();
                }
            }
        }
    }

    pub fn draw(&self) {
        if let Some(state) = self.states.last() {
            state.draw();
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

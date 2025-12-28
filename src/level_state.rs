use std::sync::Arc;
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams};
use macroquad::math::{vec2, IVec2, Rect};
use macroquad::color::WHITE;
use crate::game_state::{GameState, MenuState, Player, PlayerInitialInfo, StateTransition};
use crate::{index_to_coords, level};
use crate::controls::CONTROLS;
use crate::menu::{menu_centre_pos, Menu, MenuItem};
use crate::resources::RESOURCE_MANAGER;

#[derive(Clone)]
pub struct LevelState {
    pub level: Arc<level::Level>,
    pub player: Player,
}

impl LevelState {
    pub fn build(
        level: &str,
        player_info: PlayerInitialInfo,
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
        //pausing
        {
            let pause_menu_pos = menu_centre_pos(6, 2);
            let mut pause_menu = Menu::new(pause_menu_pos.x, pause_menu_pos.y);
            let title = MenuItem::new("pause", || StateTransition::None, false);
            pause_menu.add_item(title);
            let start_game = MenuItem::new(
                "Resume",
                move || StateTransition::Pop,
                true,
            );
            pause_menu.add_item(start_game);
            let menu_state = MenuState::new(pause_menu);
            let mut input = CONTROLS.lock().unwrap();
            if input.controls_enter() {
                return StateTransition::Push(Box::new(menu_state))
            }
        }
        
        let frame_time = get_frame_time();
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
        //todo FIX THIS TO ALLOW DIFFERENT LEVELS
        let new_level = format!(
            "a1_{}_{}",
            self.level.x_coord + offset.x,
            self.level.y_coord + offset.y
        );

        let player_info = PlayerInitialInfo {
            pos: new_player_pos,
            velocity: self.player.velocity,
            state: self.player.state.clone(),
        };

        // Load new LevelState
        match LevelState::build(&new_level, player_info) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state \"{}\": {err}", &new_level);
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

    fn transparent(&self) -> bool {
        false
    }
}
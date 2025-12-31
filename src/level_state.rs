use crate::controls::CONTROLS;
use crate::game_state::{GameState, MenuState, Player, PlayerInitialInfo, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::npc::NpcInGame;
use crate::resources::{Checkpoint, RESOURCE_MANAGER};
use crate::{index_to_coords, level};
use macroquad::color::WHITE;
use macroquad::math::{IVec2, Rect, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex, get_frame_time};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct LevelState {
    pub area: String,
    pub room: Arc<level::Room>,
    pub player: Player,
    pub npcs: Vec<NpcInGame>,
    pub checkpoint: Option<Checkpoint>,
}

impl LevelState {
    pub fn build(
        level: &str,
        player_info: PlayerInitialInfo,
    ) -> Result<LevelState, Box<dyn std::error::Error>> {
        let (area, room_name) = level.split_once('_').unwrap();
        let res = RESOURCE_MANAGER.lock()?;
        if let Some(room) = res.get_room(level) {
            let player = Player::new_from_info(player_info, room.clone());
            let area_npcs: HashMap<String, Vec<NpcInGame>>;
            let area_objects = res.get_room_object(area).unwrap();
            area_npcs = area_objects.npcs.clone();
            let npcs: Vec<NpcInGame>;
            if area_npcs.contains_key(room_name) {
                npcs = area_npcs.get(room_name).unwrap().clone();
            } else {
                npcs = vec![];
            }
            let area_checkpoints = area_objects.checkpoints.clone();
            let checkpoint: Option<Checkpoint>;
            if area_checkpoints.contains_key(room_name) {
                checkpoint = Some(area_checkpoints.get(room_name).unwrap().clone());
            } else {
                checkpoint = None;
            }
            Ok(LevelState {
                area: level.split('_').collect::<Vec<&str>>()[0].to_string(),
                room: room,
                player,
                npcs: npcs,
                checkpoint,
            })
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
        // println!("{}", self.area);
        //pausing
        {
            let pause_menu_pos = menu_centre_pos(6, 3);
            let mut pause_menu = Menu::new(pause_menu_pos.x, pause_menu_pos.y);
            let title = MenuItem::new("pause", || StateTransition::None, false);
            pause_menu.add_item(title);
            let resume_game = MenuItem::new("Resume", move || StateTransition::Pop(1), true);
            pause_menu.add_item(resume_game);
            let quit_game = MenuItem::new("Quit", move || StateTransition::Pop(2), true);
            pause_menu.add_item(quit_game);
            let menu_state = MenuState::new(pause_menu);
            let mut input = CONTROLS.lock().unwrap();
            if input.controls_enter() {
                return StateTransition::Push(Box::new(menu_state));
            }
        }

        let frame_time = get_frame_time();
        self.player.update(frame_time);

        let map_width = (self.room.map_dimensions.x * self.room.tile_size) as f32;
        let map_height = (self.room.map_dimensions.y * self.room.tile_size) as f32;

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
            //check npc player interact
            let mut input = CONTROLS.lock().unwrap();
            let player = &self.player;
            for npc in &self.npcs {
                let overlapping_x = player.position.x < npc.pos.x + npc.size.x
                    && player.position.x + player.actual_size.x as f32 > npc.pos.x;

                let overlapping_y = player.position.y < npc.pos.y + npc.size.y
                    && player.position.y + player.actual_size.y as f32 > npc.pos.y;

                if overlapping_x && overlapping_y && input.controls_up() {
                    return npc.interact();
                }
            }
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
            "{}_{}_{}",
            self.area,
            self.room.x_coord + offset.x,
            self.room.y_coord + offset.y
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

        let level = &self.room;

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
        //draw npcs
        for npc in &self.npcs {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("npc.png");
            draw_texture_ex(
                tex,
                npc.pos.x,
                npc.pos.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }
        if let Some(checkpoint) = &self.checkpoint {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("checkpoint.png");
            draw_texture_ex(
                tex,
                checkpoint.pos_x,
                checkpoint.pos_y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }
    }

    fn transparent(&self) -> bool {
        false
    }
}

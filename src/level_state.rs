use crate::controls::CONTROLS;
use crate::game_state::{
    GameState, MenuState, Player, PlayerInitialInfo, SaveData, StateTransition,
};
use crate::hud::{MapPixelType, draw_hud, get_map_pixels};
use crate::level;
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::obj__trait::Obj;
use crate::obj_checkpoint::Checkpoint;
use crate::obj_door::DoorInGame;
use crate::obj_npc::NpcInGame;
use crate::resources::RESOURCE_MANAGER;
use macroquad::color::WHITE;
use macroquad::math::{IVec2, Rect, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex, get_frame_time};
use std::collections::HashMap;
use std::env::current_exe;
use std::fs;
use std::sync::Arc;

#[derive(Clone)]
pub struct SaveGameState {
    checkpoint: Checkpoint,
}
impl SaveGameState {
    pub fn new(checkpoint: &Checkpoint) -> Self {
        SaveGameState {
            checkpoint: checkpoint.clone(),
        }
    }
}

impl GameState for SaveGameState {
    fn update(&mut self) -> StateTransition {
        use rfd::FileDialog;
        let mut exe_path = current_exe().unwrap();
        exe_path.pop(); //remove the executable filename
        let saves_path = exe_path.join("../../saves"); //temporary for when working on game? may need to change

        let file = FileDialog::new()
            .add_filter("game_25 save", &["save"])
            .set_directory(saves_path)
            .set_title("Choose file to save over")
            .pick_file();
        let new_save = SaveData {
            area: self.checkpoint.area.clone(),
            checkpoint: self.checkpoint.id,
        };

        if let Some(path) = file {
            let json_data = serde_json::to_string_pretty(&new_save).unwrap();
            println!("{}", path.display());
            fs::write(path, json_data);
        }
        StateTransition::Pop(2)
    }
    fn draw(&self) {}
    fn transparent(&self) -> bool {
        true
    }
}

pub struct LevelState {
    pub area: String,
    pub room: Arc<level::Room>,
    pub player: Player,
    pub objects: Vec<Arc<dyn Obj>>,
    pub map_pixels: Vec<Vec<MapPixelType>>,
    pub full_hud: bool,
    pub hud_init_timer: f32,
    pub init_timer_done: bool,
    pub hud_hint_text: String,
    pub current_frame: usize,
    pub time_since_frame_change: f32,
}

impl LevelState {
    pub fn build(
        level: &str,
        player_info: PlayerInitialInfo,
    ) -> Result<LevelState, Box<dyn std::error::Error>> {
        let (area, room_name) = level.split_once('_').unwrap();
        // println!("{}", room_name);
        let map_pixels = get_map_pixels(area, room_name);
        let res = RESOURCE_MANAGER.lock()?;
        if let Some(room) = res.get_room(level) {
            let player = Player::new_from_info(player_info, room.clone());
            let area_objects = res.get_room_object(area).unwrap();
            let objects: Vec<Arc<dyn Obj>>;
            if let Some(objects_from_res) = area_objects.objects.get(room_name) {
                objects = objects_from_res.clone();
            } else {
                objects = vec![];
            }

            Ok(LevelState {
                area: area.to_string(),
                room: room,
                player,
                map_pixels,
                full_hud: true,
                hud_init_timer: 2.,
                init_timer_done: false,
                hud_hint_text: String::from(""),
                current_frame: 0,
                time_since_frame_change: 0.,
                objects,
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
        self.hud_hint_text = String::from("");

        //update the animation time
        self.time_since_frame_change += get_frame_time();
        if self.time_since_frame_change > 1. / 12. {
            self.time_since_frame_change = 0.;
            self.current_frame += 1;
            if self.current_frame >= 12 {
                self.current_frame = 0;
            }
        }

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
            if input.controls_enter_release() {
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
            let mut input = CONTROLS.lock().unwrap();
            let player = &self.player;
            for obj in &self.objects {
                let overlapping_x = player.position.x < obj.get_pos().x + obj.get_size().x
                    && player.position.x + player.actual_size.x as f32 > obj.get_pos().x;

                let overlapping_y = player.position.y < obj.get_pos().y + obj.get_size().y
                    && player.position.y + player.actual_size.y as f32 > obj.get_pos().y;

                if overlapping_x && overlapping_y {
                    let text = obj.get_hud_text();
                    self.hud_hint_text = text.replace("KEY", input.key_string("z").as_str());
                    obj.contact();
                    if input.controls_tertirary_release() {
                        return obj.interact();
                    }
                }
            }

            if !self.init_timer_done {
                self.hud_init_timer -= frame_time;
                if self.full_hud && self.hud_init_timer <= 0. {
                    self.full_hud = false;
                    self.init_timer_done = true;
                }
            }
            if input.controls_esc_release() {
                self.full_hud = !self.full_hud;
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
                            level.get_tile_texture(i, self.current_frame).x * t_size,
                            level.get_tile_texture(i, self.current_frame).y * t_size,
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
            x = 0;
            y += 1;
        }
        for obj in &self.objects {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture(obj.get_tex());
            draw_texture_ex(
                tex,
                obj.get_pos().x,
                obj.get_pos().y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }

        //draw player
        self.player.draw();
        //draw the HUD
        let player_near_top: bool;
        if self.player.position.y < (240. - (self.player.actual_size.y as f32 / 2.)) {
            player_near_top = true;
        } else {
            player_near_top = false;
        }

        draw_hud(
            player_near_top,
            self.full_hud,
            &self.map_pixels,
            &self.hud_hint_text,
        );
    }

    fn transparent(&self) -> bool {
        return false;
    }
}

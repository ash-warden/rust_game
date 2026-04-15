use crate::area::Area;
use crate::controls::CONTROLS;
use crate::current_game::CURRENT_GAME_MANAGER;
use crate::game_state::{
    GameState, MenuState, Player, PlayerInitialInfo, SaveData, StateTransition,
};
use crate::hud::{MapPixelType, draw_hud, get_map_pixels};
use crate::level;
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::obj_checkpoint::Checkpoint;
use crate::resources::{Resources, RoomObjects, get_text};
use crate::traits_for_obj::Obj;
use macroquad::color::{Color, WHITE};
use macroquad::math::{IVec2, Rect, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex, get_frame_time};
use rfd::FileDialog;
use std::collections::HashSet;
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
        //get stars
        let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        let stars_file = cur_game.get_stars();
        let mut stars: HashSet<(String, i32)> = HashSet::new();

        stars.extend(stars_file.iter().cloned());

        let dialog_text = get_text("save_dialog");

        let mut exe_path = current_exe().unwrap();
        exe_path.pop(); //remove the executable filename
        let saves_path = exe_path.join("../../saves"); //temporary for when working on game? may need to change

        let file = FileDialog::new()
            .add_filter("game_25 save", &["save"])
            .set_directory(saves_path)
            .set_title(dialog_text)
            .pick_file();
        let new_save = SaveData {
            area: self.checkpoint.area.clone(),
            checkpoint: self.checkpoint.id,
            stars_collected: stars,
        };

        if let Some(path) = file {
            let json_data = serde_json::to_string_pretty(&new_save).unwrap();
            println!("{}", path.display());
            let _ = fs::write(path, json_data);
        }
        StateTransition::Pop(2)
    }
    fn draw(&self) {}
    fn transparent(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct LevelState {
    // pub area_name: String,
    pub room: Arc<level::Room>,
    pub player: Player,
    // pub objects: Vec<Arc<dyn Obj>>,
    pub map_pixels: Vec<Vec<MapPixelType>>,
    pub full_hud: bool,
    pub hud_init_timer: f32,
    pub init_timer_done: bool,
    pub hud_hint_text: String,
    pub current_frame: usize,
    pub time_since_frame_change: f32,
    pub area: Area,
}

impl LevelState {
    pub fn build(
        level: &str,
        player_info: PlayerInitialInfo,
    ) -> Result<LevelState, Box<dyn std::error::Error>> {
        let (area_name, room_name) = level.split_once('_').unwrap();
        let area = Area::build(level);
        // println!("{}", room_name);
        let map_pixels = get_map_pixels(area_name, room_name);
        let res = Resources::global();
        if let Some(room) = res.get_room(level) {
            let player = Player::new_from_info(player_info, room.clone());
            // let area_objects = res.get_room_object(area_name).unwrap();
            // let objects: Vec<Arc<dyn Obj>>;
            // if let Some(objects_from_res) = area_objects.objects.get(room_name) {
            //     objects = objects_from_res.clone();
            // } else {
            //     objects = vec![];
            // }

            Ok(LevelState {
                // area_name: area.to_string(),
                room: room,
                player,
                map_pixels,
                full_hud: true,
                hud_init_timer: 2.,
                init_timer_done: false,
                hud_hint_text: String::from(""),
                current_frame: 0,
                time_since_frame_change: 0.,
                area: area,
                // objects,
            })
        } else {
            Err("Level not found in resources".into())
        }
    }
}

pub fn area_colour(area: &str) -> Color {
    match area {
        "a1" => Color::from_hex(0x5398eb),
        _ => WHITE,
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

        //update the animation time ASSUMES THERE ARE 12 FRAMES
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
            let pause_text = get_text("pause");
            let resume_text = get_text("resume");
            let quit_text = get_text("quit");

            let pause_menu_pos = menu_centre_pos(6, 3);
            let mut pause_menu = Menu::new(pause_menu_pos.x, pause_menu_pos.y);
            let title = MenuItem::new(&pause_text, || StateTransition::None, false);
            pause_menu.add_item(title);
            let resume_game = MenuItem::new(&resume_text, move || StateTransition::Pop(1), true);
            pause_menu.add_item(resume_game);
            let quit_game = MenuItem::new(&quit_text, move || StateTransition::Pop(2), true);
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

        let p_size_x = self.player.actual_size.x as f32;
        let p_size_y = self.player.actual_size.y as f32;

        let direction = if self.player.position.x < -p_size_x / 2. {
            DirectionToMove::Left
        } else if self.player.position.x > map_width - p_size_x / 2. {
            DirectionToMove::Right
        } else if self.player.position.y < -p_size_y / 2. {
            DirectionToMove::Up
        } else if self.player.position.y > map_height - p_size_y / 2. {
            DirectionToMove::Down
        } else {
            DirectionToMove::None
        };

        if direction == DirectionToMove::None {
            let mut input = CONTROLS.lock().unwrap();
            let player = &self.player;
            if let Some(objects) = self
                .area
                .get_obj_for_room(self.room.x_coord, self.room.y_coord)
            {
                for obj in objects {
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
                vec2(map_width - p_size_x / 2. - 1., self.player.position.y),
                IVec2::new(-1, 0),
            ),
            DirectionToMove::Right => (
                vec2(-p_size_x / 2. + 1., self.player.position.y - 1.),
                IVec2::new(1, 0),
            ),
            DirectionToMove::Up => (
                vec2(self.player.position.x, map_height - p_size_y / 2. - 1.),
                IVec2::new(0, 1),
            ),
            DirectionToMove::Down => (
                vec2(self.player.position.x, -p_size_y / 2. + 1.),
                IVec2::new(0, -1),
            ),
            DirectionToMove::None => unreachable!(),
        };

        let new_room = format!(
            "{}_{}_{}",
            self.area.area_name,
            self.room.x_coord + offset.x,
            self.room.y_coord + offset.y
        );

        let player_info = PlayerInitialInfo {
            pos: new_player_pos,
            velocity: self.player.velocity,
            state: self.player.state.clone(),
        };

        // Load new LevelState
        match LevelState::build(&new_room, player_info) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state \"{}\": {err}", &new_room);
                std::process::exit(1);
            }
        }
    }

    fn draw(&self) {
        //draw tiles
        let mut i = 0; //tile number
        let mut x = 0; //x coord
        let mut y = 0; //y coord

        let room = &self.room;

        let map_width = room.map_dimensions.x;
        let map_height = room.map_dimensions.y;

        let t_size = room.tile_size as f32;

        while y < map_height {
            //column
            while x < map_width {
                //row
                let res = Resources::global();
                let tex = res.get_texture(&room.tile_image_name);
                draw_texture_ex(
                    tex,
                    x as f32 * t_size,
                    y as f32 * t_size,
                    area_colour(&self.area.area_name),
                    DrawTextureParams {
                        dest_size: Some(vec2(t_size, t_size)),
                        source: Some(Rect::new(
                            room.get_tile_texture(i, self.current_frame).x * t_size,
                            room.get_tile_texture(i, self.current_frame).y * t_size,
                            t_size,
                            t_size,
                        )),
                        ..Default::default()
                    },
                );
                x += 1;
                if i >= room.tile_values.len() {
                    println!("too many tiles to draw!");
                    break;
                }
                i += 1;
            }
            x = 0;
            y += 1;
        }

        let current_frame_f = self.current_frame as f32;
        if let Some(objects) = self
            .area
            .get_obj_for_room(self.room.x_coord, self.room.y_coord)
        {
            for obj in objects {
                obj.draw(current_frame_f);
            }
        }

        //draw player
        self.player.draw(self.current_frame as i32);
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

// name in json, name of struct
// @start_room_object_info
// doors,DoorFromFile
// @end_room_object_info

use std::sync::Arc;

use macroquad::{
    color::Color,
    math::{IVec2, Rect, Vec2, ivec2, vec2},
    texture::{DrawTextureParams, draw_texture_ex},
};
use serde::{Deserialize, Serialize};

use crate::{
    game_state::StateTransition,
    level_state::{LevelState, area_colour},
    player::{PlayerInitialInfo, PlayerMovementState},
    resources::{Resources, get_text},
    text::write_text,
    traits_for_obj::{FileToInGame, Obj},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DoorDestination {
    pub area: String,
    pub room_x: i32,
    pub room_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}
#[derive(Clone, Debug)]
pub struct DoorInGame {
    pub pos: Vec2,
    pub size: Vec2,
    pub destination: DoorDestination,
    pub visible: bool,
    pub need_interact: bool,
}

impl DoorInGame {
    pub fn new(
        pos: Vec2,
        destination: DoorDestination,
        visible: bool,
        need_interact: bool,
    ) -> Self {
        DoorInGame {
            pos,
            size: vec2(32., 64.),
            destination,
            visible,
            need_interact,
        }
    }
}

impl Obj for DoorInGame {
    fn interact(&self) -> StateTransition {
        println!(
            "entering door to {} {} {}",
            self.destination.area, self.destination.room_x, self.destination.room_y
        );
        let new_level = format!(
            "{}_{}_{}",
            self.destination.area, self.destination.room_x, self.destination.room_y,
        );
        let player_info = PlayerInitialInfo {
            pos: vec2(
                self.destination.pos_x as f32 * 32.,
                self.destination.pos_y as f32 * 32.,
            ),
            velocity: vec2(0., 0.),
            state: PlayerMovementState::Standing,
        };
        match LevelState::build(&new_level, player_info) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state \"{}\": {err}", &new_level);
                std::process::exit(1);
            }
        }
    }

    fn contact(&self) -> StateTransition {
        if !self.need_interact {
            self.interact()
        } else {
            StateTransition::None
        }
    }
    fn get_pos(&self) -> Vec2 {
        self.pos
    }
    fn get_size(&self) -> Vec2 {
        self.size
    }
    fn get_hud_text(&self) -> &str {
        get_text("obj_door_hud")
    }

    fn get_tex(&self) -> &str {
        "door.png"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn draw(&self, current_frame: f32) {
        if self.is_visible() {
            let dest_color = Color::from_vec(area_colour(&self.destination.area).to_vec() * 1.7);
            let res = Resources::global();
            let tex = res.get_texture(self.get_tex());
            draw_texture_ex(
                tex,
                self.get_pos().x,
                self.get_pos().y,
                dest_color,
                DrawTextureParams {
                    source: Some(Rect::new(
                        current_frame * self.get_size().x,
                        0.,
                        self.get_size().x,
                        self.get_size().y,
                    )),
                    ..Default::default()
                },
            );
            write_text(
                &self.destination.area,
                self.get_pos() - vec2(0., 32.),
                dest_color,
                "font.png",
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DoorLocation {
    room_x: i32,
    room_y: i32,
    pos_x: i32,
    pos_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoorFromFile {
    id: i32,
    location: DoorLocation,
    destination: DoorDestination,
    visible: bool,
    need_interact: bool,
}

impl FileToInGame for DoorFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.location.room_x, self.location.room_y)
    }

    fn to_obj(&self, _area_name: &str) -> Arc<dyn Obj> {
        Arc::new(DoorInGame::new(
            vec2(
                self.location.pos_x as f32 * 32.,
                self.location.pos_y as f32 * 32.,
            ),
            self.destination.clone(),
            self.visible,
            self.need_interact,
        ))
    }
}

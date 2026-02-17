// name in json, name of struct
// @start_room_object_info
// stars,StarFromFile
// @end_room_object_info

// room, position.
// ID should start at 0 in each area. keep track of in the current game
// as a tuple or something (area, id)

use std::sync::Arc;

use macroquad::math::{Vec2, ivec2, vec2};
use serde::{Deserialize, Serialize};

use crate::{
    current_game::CURRENT_GAME_MANAGER,
    game_state::StateTransition,
    resources::get_text,
    traits_for_obj::{FileToInGame, Obj},
};

#[derive(Debug, Clone)]
pub struct StarInGame {
    pub id: i32,
    pub area: String,
    pub pos: Vec2,
    pub size: Vec2,
}

impl StarInGame {
    pub fn new(id: i32, area: String, pos: Vec2) -> StarInGame {
        let size = vec2(32., 32.);
        StarInGame {
            id,
            area,
            pos,
            size,
        }
    }
    fn is_collected(&self) -> bool {
        let collected: bool;
        {
            let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
            collected = cur_game.is_star_collected(&self.area, self.id);
        }
        return collected;
    }
}

impl Obj for StarInGame {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_hud_text(&self) -> &str {
        match self.is_collected() {
            true => "",
            false => get_text("obj_star_collected"),
        }
    }

    fn get_tex(&self) -> &str {
        "star.png"
    }

    fn contact(&self) -> crate::game_state::StateTransition {
        {
            let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
            let collected = cur_game.is_star_collected(&self.area, self.id);

            if !collected {
                cur_game.collect_star(&self.area, self.id)
            }
        }
        StateTransition::None
    }

    fn interact(&self) -> crate::game_state::StateTransition {
        StateTransition::None
    }

    fn is_visible(&self) -> bool {
        return !self.is_collected();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarFromFile {
    pub id: i32,
    pub room_x: i32,
    pub room_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl FileToInGame for StarFromFile {
    fn room_coords(&self) -> macroquad::prelude::IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, area_name: &str) -> std::sync::Arc<dyn Obj> {
        Arc::new(StarInGame::new(
            self.id,
            area_name.to_string(),
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
        ))
    }
}

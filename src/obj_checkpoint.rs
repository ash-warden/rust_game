// name in json, name of struct
// @start_room_object_info
// checkpoints,CheckpointFromFile
// @end_room_object_info

use std::sync::Arc;

use macroquad::math::{IVec2, Vec2, ivec2, vec2};
use serde::{Deserialize, Serialize};

use crate::{
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    game_state::{MenuState, StateTransition},
    level_state::SaveGameState,
    menu::{Menu, MenuItem, menu_centre_pos},
    resources::{RESOURCE_MANAGER, get_text},
    traits_for_obj::{FileToInGame, Obj},
};

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: i32,
    pub area: String,
    pub pos: Vec2,
    pub size: Vec2,
}

impl Checkpoint {
    pub fn new(id: i32, area: String, pos: Vec2) -> Checkpoint {
        let size = vec2(32., 32.);
        Checkpoint {
            id,
            area,
            pos,
            size,
        }
    }
}

impl Obj for Checkpoint {
    fn contact(&self) -> StateTransition {
        let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        cur_game.health = MAX_HEALTH;
        StateTransition::None
    }

    fn interact(&self) -> StateTransition {
        let save_game_text = get_text("save_game");
        let yes_text = get_text("yes");
        let no_text = get_text("no");

        println!("{}", "interacting with checkpoint");
        let save_game_state = SaveGameState::new(self);
        let menu_pos = menu_centre_pos(15, 3);
        let mut menu = Menu::new(menu_pos.x, menu_pos.y);
        let title = MenuItem::new(&save_game_text, || StateTransition::None, false);
        menu.add_item(title);
        let save_game = MenuItem::new(
            &yes_text,
            move || StateTransition::Push(Box::new(save_game_state.clone())),
            true,
        );
        menu.add_item(save_game);
        let cancel = MenuItem::new(&no_text, move || StateTransition::Pop(1), true);
        menu.add_item(cancel);
        let menu_state = MenuState::new(menu);
        StateTransition::Push(Box::new(menu_state))
    }
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }
    fn get_hud_text(&self) -> String {
        get_text("obj_check_hud")
    }

    fn get_tex(&self) -> &str {
        "checkpoint.png"
    }

    fn is_visible(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckpointFromFile {
    pub id: i32,
    pub room_x: i32,
    pub room_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl FileToInGame for CheckpointFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, area_name: &str) -> Arc<dyn Obj> {
        Arc::new(Checkpoint::new(
            self.id,
            area_name.to_string(),
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
        ))
    }
}

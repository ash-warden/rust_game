// name in json, name of struct
// @start_room_object_info
// npcs,NpcFromFile
// @end_room_object_info

use std::sync::Arc;

use crate::game_state::{MenuState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::traits_for_obj::{FileToInGame, Obj};
use macroquad::math::{IVec2, Vec2, ivec2, vec2};
use serde::{Deserialize, Serialize};

fn simple_dialog(text: &str) -> StateTransition {
    let dialog_menu_pos = menu_centre_pos(36, 1000); //h not used
    let mut dialog_menu = Menu::new(dialog_menu_pos.x, 300.);
    let text = MenuItem::new(text, || StateTransition::None, false);
    dialog_menu.add_item(text);
    let next = MenuItem::new(">", move || StateTransition::Pop(1), true);
    dialog_menu.add_item(next);
    let menu_state = MenuState::new(dialog_menu);
    StateTransition::Push(Box::new(menu_state))
}

fn npc_function(npc_name: &str) -> StateTransition {
    match npc_name {
        "Test1" => StateTransition::None,
        _ => simple_dialog("Error, NPC has no function"),
    }
}

#[derive(Clone, Debug)]
pub struct NpcInGame {
    pub pos: Vec2,
    pub size: Vec2,
    pub npc_type: String,
}

impl NpcInGame {
    pub fn new(pos: Vec2, npc_type: String) -> Self {
        NpcInGame {
            pos,
            size: vec2(32., 64.),
            npc_type,
        }
    }
}

impl Obj for NpcInGame {
    fn interact(&self) -> StateTransition {
        npc_function(&self.npc_type)
    }

    fn contact(&self) -> StateTransition {
        StateTransition::None
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }
    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_hud_text(&self) -> &str {
        "Press KEY to\ntalk"
    }

    fn get_tex(&self) -> &str {
        "npc.png"
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NpcFromFile {
    id: i32,
    name: String,
    room_x: i32,
    room_y: i32,
    pos_x: i32,
    pos_y: i32,
}

impl FileToInGame for NpcFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, _area_name: &str) -> Arc<dyn Obj> {
        Arc::new(NpcInGame::new(
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
            self.name.clone(),
        ))
    }
}

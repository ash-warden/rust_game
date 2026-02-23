// name in json, name of struct
// @start_room_object_info
// npcs,NpcFromFile
// @end_room_object_info

use std::sync::Arc;

use crate::controls::CONTROLS;
use crate::game_state::{GameState, MenuState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::resources::{Resources, get_text};
use crate::text::write_text;
use crate::traits_for_obj::{FileToInGame, Obj};
use macroquad::color::WHITE;
use macroquad::math::{IVec2, Vec2, ivec2, vec2};
use serde::{Deserialize, Serialize};

pub struct TalkState {
    pub lines: Vec<String>,
    pub function: Option<Box<dyn Fn() -> ()>>,
    pub current_line: i32,
}

impl TalkState {
    pub fn new(lines_name: &str, fun: Option<Box<dyn Fn() -> ()>>) -> Self {
        TalkState {
            lines: Resources::global().get_npc_dialog(lines_name).unwrap(),
            function: fun,
            current_line: 0,
        }
    }
}

impl GameState for TalkState {
    fn update(&mut self) -> StateTransition {
        let mut controls = CONTROLS.lock().unwrap();
        if controls.controls_tertirary_release() {
            if (self.current_line as usize) < (self.lines.len() - 1) {
                self.current_line += 1;
            } else {
                return StateTransition::Pop(1);
            }
        }
        StateTransition::None
    }
    fn draw(&self) {
        write_text(
            &self.lines[self.current_line as usize],
            vec2(32., 300.),
            WHITE,
            "font.png",
        );
    }
    fn transparent(&self) -> bool {
        true
    }
}

// GET RID OF THIS
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
        "Test1" => StateTransition::Push(Box::new(TalkState::new("test1", None))),
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
        get_text("obj_npc_hud")
    }

    fn get_tex(&self) -> &str {
        "npc.png"
    }

    fn is_visible(&self) -> bool {
        true
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

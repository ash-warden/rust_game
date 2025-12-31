use crate::game_state::{MenuState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use macroquad::math::{Vec2, vec2};

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
    pub fn interact(&self) -> StateTransition {
        npc_function(&self.npc_type)
    }
}

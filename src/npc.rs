use crate::game_state::{MenuState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use macroquad::math::Vec2;

enum Npcs {
    Test1,
    Test2,
}

fn npc_function(npc: Npcs) -> StateTransition {
    match npc {
        Npcs::Test1 => StateTransition::None,
        _ => {
            let dialog_menu_pos = menu_centre_pos(26, 2);
            let mut dialog_menu = Menu::new(dialog_menu_pos.x, dialog_menu_pos.y);
            let text = MenuItem::new(
                "Error, NPC has no function",
                || StateTransition::None,
                false,
            );
            dialog_menu.add_item(text);
            let next = MenuItem::new(">", move || StateTransition::Pop(1), true);
            dialog_menu.add_item(next);
            let menu_state = MenuState::new(dialog_menu);
            StateTransition::Push(Box::new(menu_state))
        }
    }
}

#[derive(Clone)]
pub struct Npc {
    pub pos: Vec2,
}

impl Npc {
    pub fn new(pos: Vec2) -> Self {
        Npc { pos }
    }
    pub fn interact(&self) -> StateTransition {
        npc_function(Npcs::Test2)
    }
}

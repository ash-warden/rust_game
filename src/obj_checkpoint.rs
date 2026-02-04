use macroquad::math::{Vec2, vec2};

use crate::{
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    game_state::{MenuState, StateTransition},
    level_state::SaveGameState,
    menu::{Menu, MenuItem, menu_centre_pos},
    obj__trait::Obj,
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
        println!("{}", "interacting with checkpoint");
        let save_game_state = SaveGameState::new(self);
        let menu_pos = menu_centre_pos(15, 3);
        let mut menu = Menu::new(menu_pos.x, menu_pos.y);
        let title = MenuItem::new("Save game file?", || StateTransition::None, false);
        menu.add_item(title);
        let save_game = MenuItem::new(
            "Yes",
            move || StateTransition::Push(Box::new(save_game_state.clone())),
            true,
        );
        menu.add_item(save_game);
        let cancel = MenuItem::new("No", move || StateTransition::Pop(1), true);
        menu.add_item(cancel);
        let menu_state = MenuState::new(menu);
        StateTransition::Push(Box::new(menu_state))
    }
}

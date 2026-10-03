//NOTE
// check obj_npc, similar situation with there being multiple types of the thing

use crate::{game_state::StateTransition, message_window::MsgState};

//item in the inventory
#[derive(Clone)]
pub struct ItemInv {
    pub name: String,
    pub can_drop: bool,
    use_hint: String,
    destroy_after_use: bool,
    pub is_destroyed:bool,
}

impl ItemInv {
    pub fn new(
        name: String,
        use_hint: String,
        can_drop: bool,
        destroy_after_use: bool,
    ) -> Self {
        ItemInv {
            name,
            use_hint,
            can_drop,
            destroy_after_use,
            is_destroyed: false,
        }
    }

    pub fn item_function(&mut self) -> StateTransition {
        let item_name = self.name.as_str();
        match item_name {
            "test_item" => {
                self.is_destroyed = true;
                return StateTransition::Push(Box::new(MsgState::new("Used the test item", 3)))
            }
            _ => {
                println!("Error using item, item not found");
            }
        }
        StateTransition::Pop(1)
    }
}

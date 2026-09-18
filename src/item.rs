//NOTE
// check obj_npc, similar situation

use crate::game_state::StateTransition;

//item in the inventory
pub struct ItemInv {
    pub name: String,
    can_drop: bool,
    use_primary_hint: String,
    use_second_hint: String,
}

impl ItemInv {
    pub fn new(name: String, use_primary_hint: String, use_second_hint: String, can_drop: bool) -> Self {
        ItemInv {
            name,
            use_primary_hint,
            use_second_hint,
            can_drop,
        }
    }
}

fn item_function(item_name: &str) -> StateTransition {
    match item_name {
        "test_item1" => {
            println!("using test item");
            StateTransition::None
        }
        _ => {
            println!("Error using item, item not found");
            StateTransition::None
        }
    }
}

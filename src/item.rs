//NOTE
// check obj_npc, similar situation with there being multiple types of the thing

use crate::game_state::StateTransition;

//item in the inventory
#[derive(Clone)]
pub struct ItemInv {
    pub name: String,
    pub can_drop: bool,
    use_primary_hint: String,
    use_second_hint: String,
}

impl ItemInv {
    pub fn new(
        name: String,
        use_primary_hint: String,
        use_second_hint: String,
        can_drop: bool,
    ) -> Self {
        ItemInv {
            name,
            use_primary_hint,
            use_second_hint,
            can_drop,
        }
    }

    pub fn item_function(&self) {
        let item_name = self.name.as_str();
        match item_name {
            "test_item" => {
                println!("using test item");
            }
            _ => {
                println!("Error using item, item not found");
            }
        }
    }
}

use crate::game_state::StateTransition;

pub trait Obj {
    fn contact(&self) -> StateTransition;
    fn interact(&self) -> StateTransition;
}

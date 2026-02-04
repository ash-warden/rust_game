use std::sync::Arc;

use macroquad::math::{IVec2, Vec2};

use crate::game_state::StateTransition;

pub trait FileToInGame {
    fn room_coords(&self) -> IVec2;
    fn to_obj(&self, area_name: &str) -> Arc<dyn Obj>;
}

pub trait Obj: Send + Sync {
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_hud_text(&self) -> &str;
    fn get_tex(&self) -> &str;
    fn contact(&self) -> StateTransition;
    fn interact(&self) -> StateTransition;
}

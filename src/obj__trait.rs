use macroquad::math::Vec2;

use crate::game_state::StateTransition;

pub trait Obj: Send + Sync {
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_hud_text(&self) -> &str;
    fn get_tex(&self) -> &str;
    fn contact(&self) -> StateTransition;
    fn interact(&self) -> StateTransition;
}

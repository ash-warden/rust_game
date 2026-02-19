use std::sync::Arc;

use macroquad::{
    color::WHITE,
    math::{IVec2, Rect, Vec2},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{game_state::StateTransition, resources::Resources};

pub trait FileToInGame {
    fn room_coords(&self) -> IVec2;
    fn to_obj(&self, area_name: &str) -> Arc<dyn Obj>;
}

pub trait Obj: Send + Sync {
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_hud_text(&self) -> &str;
    fn get_tex(&self) -> &str;
    fn is_visible(&self) -> bool;
    fn contact(&self) -> StateTransition;
    fn interact(&self) -> StateTransition;
    fn draw(&self, current_frame: f32) {
        if self.is_visible() {
            let res = Resources::global();
            let tex = res.get_texture(self.get_tex());
            draw_texture_ex(
                tex,
                self.get_pos().x,
                self.get_pos().y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        current_frame * self.get_size().x,
                        0.,
                        self.get_size().x,
                        self.get_size().y,
                    )),
                    ..Default::default()
                },
            );
        }
    }
}

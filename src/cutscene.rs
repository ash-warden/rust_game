use std::sync::Arc;

use macroquad::{
    color::{BLACK, WHITE},
    math::vec2,
    texture::{DrawTextureParams, draw_texture_ex},
    time::get_frame_time,
};
use serde::{Deserialize, Serialize};

use crate::{
    game_state::{GameState, StateTransition},
    resources::RESOURCE_MANAGER,
    text::write_text,
};

#[derive(Serialize, Deserialize)]
pub struct CutsceneSlide {
    pub image_name: String,
    pub text: String,
    pub time: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Cutscene {
    pub slides: Vec<CutsceneSlide>,
}

pub struct CutsceneState {
    pub cutscene: Arc<Cutscene>,
    pub cur_slide_no: i32,
    pub slide_timer: f32,
}

impl CutsceneState {
    pub fn new(cutscene_name: &str) -> Self {
        let res = RESOURCE_MANAGER.lock().unwrap();
        let cutscene = res.get_cutscene(cutscene_name).unwrap().clone();
        CutsceneState {
            cutscene,
            cur_slide_no: 0,
            slide_timer: 0.,
        }
    }
}

impl GameState for CutsceneState {
    fn update(&mut self) -> crate::game_state::StateTransition {
        if self.cutscene.slides[self.cur_slide_no as usize].time < self.slide_timer {
            if self.cur_slide_no < self.cutscene.slides.len() as i32 - 1 {
                self.cur_slide_no += 1;
            } else {
                return StateTransition::Pop(1);
            }
            self.slide_timer = 0.;
        }
        self.slide_timer += get_frame_time();
        StateTransition::None
    }

    fn draw(&self) {
        {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture(&self.cutscene.slides[self.cur_slide_no as usize].image_name);
            draw_texture_ex(
                tex,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }
        write_text(
            &self.cutscene.slides[self.cur_slide_no as usize].text,
            vec2(0., 0.),
            BLACK,
            "font.png",
        );
    }

    fn transparent(&self) -> bool {
        false
    }
}

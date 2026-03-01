use std::sync::Arc;

use macroquad::{
    color::{Color, WHITE},
    math::vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, draw_texture_ex},
    time::get_frame_time,
};
use serde::{Deserialize, Serialize};

use crate::{
    game_state::{GameState, StateTransition},
    resources::Resources,
    text::write_text,
};

#[derive(Serialize, Deserialize)]
pub struct CutsceneSlide {
    pub image_name: String,
    pub text: String,
    pub time: f32,
    pub lines: i32,
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
        let res = Resources::global();
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
        let current_slide = &self.cutscene.slides[self.cur_slide_no as usize];
        {
            let res = Resources::global();
            let tex = res.get_texture(&current_slide.image_name);
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
        let space_needed = (480 - current_slide.lines * 32) as f32;
        draw_rectangle(
            0.,
            space_needed,
            640.,
            480. - space_needed,
            Color::new(0.1, 0.1, 0.1, 0.3),
        );
        write_text(
            &current_slide.text,
            vec2(0., space_needed),
            WHITE,
            "font.png",
        );
    }

    fn transparent(&self) -> bool {
        false
    }
}

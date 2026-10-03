use macroquad::{color::{Color, WHITE}, math::vec2, shapes::draw_rectangle};

use crate::{controls::CONTROLS, game_state::{GameState, StateTransition}, text::write_text};

pub struct MsgState {
    pub message: String,
    pop_no: i32,
}

impl MsgState {
    pub fn new (message: &str, pop_no: i32) -> MsgState {
        MsgState {
            message: message.to_owned(),
            pop_no,
        }
    }
}

impl GameState for MsgState {
    fn update(&mut self) -> crate::game_state::StateTransition {
        let mut controls = CONTROLS.lock().unwrap();
        if controls.controls_primary_release() {
            return StateTransition::Pop(self.pop_no)
        }
        StateTransition::None
    }

    fn draw(&self) {
        draw_rectangle(16., 304., 608., 160., Color::new(0.1, 0.1, 0.1, 1.));
        write_text(
            &self.message,
            vec2(32., 320.),
            WHITE,
            "font.png",
        );
    }

    fn transparent(&self) -> bool {
        true
    }
}

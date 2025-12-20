use std::sync::Mutex;
use gamepads::{Button, Gamepads};
use macroquad::input::{is_key_down, KeyCode};
use once_cell::sync::Lazy;

pub struct Controls {
    pads: Gamepads,
}

impl Controls {
    fn new() -> Self {
        Self { pads: Gamepads::new() }
    }

    fn poll(&mut self) {
        self.pads.poll();
    }

    fn check_pad_input(&mut self, button: Button) -> bool {
        self.poll();
        self.pads
            .all()
            .next()
            .map(|g| g.is_currently_pressed(button))
            .unwrap_or(false)
    }

    pub fn controls_left(&mut self) -> bool {
        self.check_pad_input(Button::DPadLeft) || is_key_down(KeyCode::Left)
    }
    pub fn controls_right(&mut self) -> bool {
        self.check_pad_input(Button::DPadRight) || is_key_down(KeyCode::Right)
    }
    pub fn controls_up(&mut self) -> bool {
        self.check_pad_input(Button::DPadUp) || is_key_down(KeyCode::Up)
    }
    pub fn controls_down(&mut self) -> bool {
        self.check_pad_input(Button::DPadDown) || is_key_down(KeyCode::Down)
    }
    pub fn controls_primary(&mut self) -> bool {
        self.check_pad_input(Button::ActionDown) || is_key_down(KeyCode::Space)
    }
    pub fn controls_secondary(&mut self) -> bool {
        self.check_pad_input(Button::ActionLeft) || is_key_down(KeyCode::LeftShift)
    }
    pub fn controls_enter(&mut self) -> bool {
        self.check_pad_input(Button::RightCenterCluster) || is_key_down(KeyCode::Enter)
    }
}

pub static CONTROLS: Lazy<Mutex<Controls>> = Lazy::new(|| Mutex::new(Controls::new()));

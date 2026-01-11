use gamepads::{Button, Gamepads};
use macroquad::input::{KeyCode, get_keys_pressed, is_key_down};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Controls {
    pads: Gamepads,
    enter_down: bool,
    z_down: bool,
    x_down: bool,
    esc_down: bool,
    last_used_controller: bool, // true if controler was used last, false if keyboard was used last
}

impl Controls {
    fn new() -> Self {
        Self {
            pads: Gamepads::new(),
            enter_down: false,
            z_down: false,
            x_down: false,
            esc_down: false,
            last_used_controller: false,
        }
    }

    fn poll(&mut self) {
        self.pads.poll();
    }

    fn check_pad_input(&mut self, button: Button) -> bool {
        self.poll();
        // check whether keyboard or contoller used last
        if self.pads.all().count() > 0 {
            if self
                .pads
                .all()
                .next()
                .unwrap()
                .all_currently_pressed()
                .count()
                > 0
            {
                self.last_used_controller = true;
            }
        }
        if !get_keys_pressed().is_empty() {
            self.last_used_controller = false;
        }
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
    pub fn controls_tertirary_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::ActionRight) || is_key_down(KeyCode::Z);
        let just_released = self.z_down && !pressed;
        self.z_down = pressed;
        just_released
    }
    pub fn controls_quaternary_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::ActionUp) || is_key_down(KeyCode::X);
        let just_released = self.x_down && !pressed;
        self.x_down = pressed;
        just_released
    }
    pub fn controls_enter_release(&mut self) -> bool {
        let pressed =
            self.check_pad_input(Button::RightCenterCluster) || is_key_down(KeyCode::Enter);
        let just_released = self.enter_down && !pressed;
        self.enter_down = pressed;
        just_released
    }
    pub fn controls_esc_release(&mut self) -> bool {
        let pressed =
            self.check_pad_input(Button::LeftCenterCluster) || is_key_down(KeyCode::Escape);
        let just_released = self.esc_down && !pressed;
        self.esc_down = pressed;
        just_released
    }
    pub fn key_string(&self, key: &str) -> String {
        let key_string: String;
        match self.last_used_controller {
            true => match key {
                "space" => key_string = String::from("A"),
                "shift" => key_string = String::from("X"),
                "z" => key_string = String::from("B"),
                "x" => key_string = String::from("Y"),
                "enter" => key_string = String::from("Start"),
                "esc" => key_string = String::from("Select"),
                _ => key_string = String::from("NO KEY"),
            },
            false => match key {
                "space" => key_string = String::from("Space"),
                "shift" => key_string = String::from("Shift"),
                "z" => key_string = String::from("Z"),
                "x" => key_string = String::from("X"),
                "enter" => key_string = String::from("Enter"),
                "esc" => key_string = String::from("Escape"),
                _ => key_string = String::from("NO KEY"),
            },
        };
        key_string
    }
}

pub static CONTROLS: Lazy<Mutex<Controls>> = Lazy::new(|| Mutex::new(Controls::new()));

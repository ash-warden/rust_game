use gilrs::{Button, GamepadId, Gilrs};
use macroquad::input::{KeyCode, get_keys_pressed, is_key_down};
use std::sync::{LazyLock, Mutex};

pub struct Controls {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    enter_down: bool,
    z_down: bool,
    x_down: bool,
    up_down: bool,
    down_down: bool,
    esc_down: bool,
    last_used_controller: bool, // true if controler was used last, false if keyboard was used last
}

impl Controls {
    fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            active_gamepad: None,
            enter_down: false,
            z_down: false,
            x_down: false,
            up_down: false,
            down_down: false,
            esc_down: false,
            last_used_controller: false,
        }
    }

    pub fn update(&mut self) {
        while let Some(event) = self.gilrs.next_event() {
            self.active_gamepad = Some(event.id);
            self.last_used_controller = true;
        }
        if !get_keys_pressed().is_empty() {
            self.last_used_controller = false;
        }
    }

    fn check_pad_input(&self, button: Button) -> bool {
        self.active_gamepad.map(|id| {
            self.gilrs.gamepad(id).is_pressed(button)
        }).unwrap_or(false)
    }

    pub fn controls_left(&self) -> bool {
        self.check_pad_input(Button::DPadLeft) || is_key_down(KeyCode::Left)
    }
    pub fn controls_right(&self) -> bool {
        self.check_pad_input(Button::DPadRight) || is_key_down(KeyCode::Right)
    }
    pub fn controls_up(&self) -> bool {
        self.check_pad_input(Button::DPadUp) || is_key_down(KeyCode::Up)
    }
    pub fn controls_down(&self) -> bool {
        self.check_pad_input(Button::DPadDown) || is_key_down(KeyCode::Down)
    }
    pub fn controls_up_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::DPadUp) || is_key_down(KeyCode::Up);
        let just_released = self.up_down && !pressed;
        self.up_down = pressed;
        just_released
    }
    pub fn controls_down_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::DPadDown) || is_key_down(KeyCode::Down);
        let just_released = self.down_down && !pressed;
        self.down_down = pressed;
        just_released
    }

    pub fn controls_primary(&mut self) -> bool {
        self.check_pad_input(Button::South) || is_key_down(KeyCode::Space)
    }
    pub fn controls_secondary(&mut self) -> bool {
        self.check_pad_input(Button::West) || is_key_down(KeyCode::LeftShift)
    }
    pub fn controls_tertirary_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::East) || is_key_down(KeyCode::Z);
        let just_released = self.z_down && !pressed;
        self.z_down = pressed;
        just_released
    }
    pub fn controls_quaternary_release(&mut self) -> bool {
        let pressed = self.check_pad_input(Button::North) || is_key_down(KeyCode::X);
        let just_released = self.x_down && !pressed;
        self.x_down = pressed;
        just_released
    }
    pub fn controls_enter_release(&mut self) -> bool {
        let pressed =
            self.check_pad_input(Button::Start) || is_key_down(KeyCode::Enter);
        let just_released = self.enter_down && !pressed;
        self.enter_down = pressed;
        just_released
    }
    pub fn controls_esc_release(&mut self) -> bool {
        let pressed =
            self.check_pad_input(Button::Select) || is_key_down(KeyCode::Escape);
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

pub static CONTROLS: LazyLock<Mutex<Controls>> = LazyLock::new(|| Mutex::new(Controls::new()));

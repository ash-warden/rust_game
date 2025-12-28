use std::env::current_exe;
use std::fs;
use macroquad::color::WHITE;
use macroquad::math::{ivec2, vec2, Rect};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
pub(crate) use crate::player::{Player, PlayerInitialInfo};
use crate::menu::Menu;
use crate::{Checkpoints, SaveData};
use crate::controls::CONTROLS;
use crate::level_state::LevelState;
use crate::player::PlayerMovementState;
use crate::resources::{RESOURCE_MANAGER};

pub enum StateTransition {
    None,
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
}

pub trait GameState {
    fn update(&mut self) -> StateTransition;
    fn draw(&self);
}

pub struct MenuState {
    pub menu: Menu,
}

impl MenuState {
    pub fn new(menu: Menu) -> Self {
        MenuState{menu}
    }
}

impl GameState for MenuState {
    fn update(&mut self) -> StateTransition {
        self.menu.update()
    }
    fn draw(&self) {
        self.menu.draw();
    }
}

#[derive(Clone)]
pub struct SillyState {
}

impl SillyState {
    pub fn new() -> Self {
        SillyState{}
    }
}

impl GameState for SillyState {
    fn update(&mut self) -> StateTransition {
        println!("nonsense!");
        StateTransition::Pop
    }
    fn draw(&self) {
    }
}

#[derive(Clone)]
pub struct PauseMenu {
}

impl PauseMenu {
    pub fn new() -> Self { PauseMenu{} }
}

impl GameState for PauseMenu {
    fn update(&mut self) -> StateTransition {
        println!("Paused");
        {
            let mut input = CONTROLS.lock().unwrap();
            if input.controls_enter() {
                StateTransition::Pop
            } else { StateTransition::None }
        }
    }
    fn draw(&self) {
        let res = RESOURCE_MANAGER.lock().unwrap();
        let tex = res.get_texture("missing.png");
        draw_texture_ex(
                tex,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(16., 16.)),
                    source: Some(Rect::new(0., 0., 16., 16.)),
                    ..Default::default()
                },
            );
    }
}

#[derive(Clone)]
pub struct LoadSaveState {
}

impl LoadSaveState {
    pub fn new() -> Self {
        LoadSaveState{}
    }
}

impl GameState for LoadSaveState {
    fn update(&mut self) -> StateTransition {
        use rfd::FileDialog;

        let mut exe_path = current_exe().unwrap();
        exe_path.pop(); //remove the executable filename
        let saves_path = exe_path.join("saves");

        let file = FileDialog::new()
            .add_filter("game_25 save", &["save"])
            .set_directory(saves_path)
            .set_title("Load save file")
            .pick_file();

        let area;
        let checkpoint: i32;
        let mut player_pos = vec2(100., 100.);
        let mut room = ivec2(0, 0);

        if let Some(path) = file {
            let contents = fs::read_to_string(&path);
            let save: SaveData = serde_json::from_str(&contents.unwrap().as_str()).expect("Error 1");

            area = save.area;
            checkpoint = save.checkpoint;

            let checkpoints_path = format!("assets/maps/{}_check.json", area);
            let checkpoints_file = fs::read_to_string(checkpoints_path);
            let checkpoints: Checkpoints = serde_json::from_str(&checkpoints_file.unwrap().as_str()).expect("Error 2");

            for i in checkpoints.checkpoints {
                if i.id == checkpoint {
                    player_pos = vec2(i.pos_x, i.pos_y);
                    room = ivec2(i.room_x, i.room_y);
                }
            }
        } else {
            println!("User cancelled the dialog");
            return StateTransition::Pop;
        }

        {
            let mut res = RESOURCE_MANAGER.lock().unwrap();
            res.scale = 1.;
        }

        let player_info = PlayerInitialInfo {
            pos: player_pos,
            velocity: vec2(0., 0.),
            state: PlayerMovementState::Standing,
        };

        let level = format!("{}_{}_{}", area, room.x, room.y);

        let level_state = LevelState::build(&level, player_info).unwrap_or_else(|err| {
            eprintln!("Failed to load level state: {err}");
            std::process::exit(1);
        });
        StateTransition::Replace(Box::new(level_state.clone()))
    }
    fn draw(&self) {
    }
}

pub struct GameStateStack {
    pub states: Vec<Box<dyn GameState>>,
}

impl GameStateStack {
    pub fn new(initial: Box<dyn GameState>) -> Self {
        Self {
            states: vec![initial],
        }
    }

    pub fn update(&mut self) {
        if let Some(state) = self.states.last_mut() {
            match state.update() {
                StateTransition::None => {}
                StateTransition::Replace(new_state) => self.replace(new_state),
                StateTransition::Push(new_state) => self.push(new_state),
                StateTransition::Pop => {
                    self.pop();
                }
            }
        }
    }

    pub fn draw(&self) {
        if let Some(state) = self.states.last() {
            state.draw();
        }
    }

    pub fn push(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn replace(&mut self, state: Box<dyn GameState>) {
        self.pop();
        self.push(state);
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

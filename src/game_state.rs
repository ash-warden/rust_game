use crate::current_game::CURRENT_GAME_MANAGER;
use crate::cutscene::CutsceneState;
use crate::level_state::LevelState;
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::player::PlayerMovementState;
pub(crate) use crate::player::{Player, PlayerInitialInfo};
use crate::room_obj_from_file::RoomObjectsFromFile;
use macroquad::math::{i32, ivec2, vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use walkdir::WalkDir;

pub enum StateTransition {
    None,
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop(i32),
}

pub trait GameState {
    fn update(&mut self) -> StateTransition;
    fn draw(&self);
    fn transparent(&self) -> bool;
}

pub struct MenuState {
    pub menu: Menu,
}

impl MenuState {
    pub fn new(menu: Menu) -> Self {
        MenuState { menu }
    }
}

impl GameState for MenuState {
    fn update(&mut self) -> StateTransition {
        self.menu.update()
    }
    fn draw(&self) {
        self.menu.draw();
    }
    fn transparent(&self) -> bool {
        true
    }
}

fn load_level_state(file: String) -> LevelState {
    let area;
    let checkpoint: i32;
    let mut player_pos = vec2(100., 100.); // value isn't actually used since it is replaced when the file is loaded
    let mut room = ivec2(0, 0);

    println!("{:?}", format!("{}{}", "saves/", file));

    let contents = fs::read_to_string(format!("{}{}{}", "saves/", file, ".save"));
    let save: SaveData = serde_json::from_str(&contents.unwrap().as_str()).expect("Error 1");

    area = save.area;
    checkpoint = save.checkpoint;

    let objects_path = format!("assets/maps/{}_objects.roomobj", area);
    let objects_file = fs::read_to_string(objects_path);
    let objects: RoomObjectsFromFile =
        serde_json::from_str(&objects_file.unwrap().as_str()).expect("Error 2");

    if let Some(checkpoints) = objects.checkpoints {
        for i in checkpoints {
            if i.id == checkpoint {
                player_pos = vec2(i.pos_x as f32 * 32., i.pos_y as f32 * 32.);
                room = ivec2(i.room_x, i.room_y);
            }
        }
    }

    let player_info = PlayerInitialInfo {
        pos: player_pos,
        velocity: vec2(0., 0.),
        state: PlayerMovementState::Standing,
    };

    let level = format!("{}_{}_{}", area, room.x, room.y);

    //set the current file name. used when saving
    let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
    cur_game.save_name = file;

    //set stars
    cur_game.stars_collected = save.stars_collected;

    LevelState::build(&level, player_info).unwrap_or_else(|err| {
        eprintln!("Failed to load level state: {err}");
        std::process::exit(1);
    })
}

// for LOADING a game
pub struct LoadSaveState {
    save_file_names: Vec<String>,
}

impl LoadSaveState {
    pub fn new() -> Self {
        let mut saves: Vec<String> = vec![];
        //get the saves
        for entry in WalkDir::new("saves") {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap_or_default() == "save" {
                saves.push(path.file_stem().unwrap().to_str().unwrap().to_owned());
            }
        }
        LoadSaveState {
            save_file_names: saves,
        }
    }
}

impl GameState for LoadSaveState {
    fn update(&mut self) -> StateTransition {
        let menu_pos = menu_centre_pos(20, 10);
        let mut menu = Menu::new(menu_pos.x, menu_pos.y);
        let title = MenuItem::new("Select a file to load", || StateTransition::None, false);
        menu.add_item(title);

        for i in 0..self.save_file_names.len() {
            let file_name = &self.save_file_names[i];
            let loaded_level_state = load_level_state(file_name.to_owned());
            let file_load = MenuItem::new(
                file_name,
                move || StateTransition::Replace(Box::new(loaded_level_state.clone())),
                true,
            );
            menu.add_item(file_load);
        }
        let cancel = MenuItem::new("Cancel", || StateTransition::Pop(2), true);
        menu.add_item(cancel);

        let menu_state = MenuState::new(menu);

        return StateTransition::Push(Box::new(menu_state));
    }

    fn draw(&self) {}

    fn transparent(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct NewGameState {
    pub cutscene_done: bool,
}

impl NewGameState {
    pub fn new() -> Self {
        let mut cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
        cur_game.reset();
        NewGameState {
            cutscene_done: false,
        }
    }
}

impl GameState for NewGameState {
    fn update(&mut self) -> StateTransition {
        if !self.cutscene_done {
            self.cutscene_done = true;
            StateTransition::Push(Box::new(CutsceneState::new("intro")))
        } else {
            let player_pos = vec2(100., 100.); // value isn't actually used since it is replaced when the file is loaded

            let player_info = PlayerInitialInfo {
                pos: player_pos,
                velocity: vec2(0., 0.),
                state: PlayerMovementState::Standing,
            };

            let level = format!("{}_{}_{}", "a1", 0, 0);

            let level_state = LevelState::build(&level, player_info).unwrap_or_else(|err| {
                eprintln!("Failed to load level state: {err}");
                std::process::exit(1);
            });
            StateTransition::Replace(Box::new(level_state))
        }
    }
    fn draw(&self) {}

    fn transparent(&self) -> bool {
        true
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
                StateTransition::Pop(number) => {
                    for _ in 0..number {
                        self.pop();
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        let mut start_index = 0;
        for (i, state) in self.states.iter().enumerate().rev() {
            if !state.transparent() {
                start_index = i;
                break;
            }
        }
        for state in &self.states[start_index..] {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub area: String,
    pub checkpoint: i32,
    pub stars_collected: HashSet<(String, i32)>,
}

pub(crate) use crate::player::{Player, PlayerInitialInfo};
use crate::menu::Menu;

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

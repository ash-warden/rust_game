use macroquad::math::{Vec2, vec2};
use serde::{Deserialize, Serialize};

use crate::game_state::StateTransition;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DoorDestination {
    pub area: String,
    pub room_x: i32,
    pub room_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}
#[derive(Clone, Debug)]
pub struct DoorInGame {
    pub pos: Vec2,
    pub size: Vec2,
    pub destination: DoorDestination,
    pub visible: bool,
    pub need_interact: bool,
}

impl DoorInGame {
    pub fn new(
        pos: Vec2,
        destination: DoorDestination,
        visible: bool,
        need_interact: bool,
    ) -> Self {
        DoorInGame {
            pos,
            size: vec2(32., 64.),
            destination,
            visible,
            need_interact,
        }
    }
    pub fn interact(&self) -> StateTransition {
        println!(
            "entering door to {} {} {}",
            self.destination.area, self.destination.room_x, self.destination.room_y
        );
        StateTransition::None
    }
}

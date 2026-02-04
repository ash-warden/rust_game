use macroquad::math::{Vec2, vec2};
use serde::{Deserialize, Serialize};

use crate::{
    game_state::StateTransition,
    level_state::LevelState,
    player::{PlayerInitialInfo, PlayerMovementState},
};

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
        let new_level = format!(
            "{}_{}_{}",
            self.destination.area, self.destination.room_x, self.destination.room_y,
        );
        let player_info = PlayerInitialInfo {
            pos: vec2(
                self.destination.pos_x as f32 * 32.,
                self.destination.pos_y as f32 * 32.,
            ),
            velocity: vec2(0., 0.),
            state: PlayerMovementState::Standing,
        };
        match LevelState::build(&new_level, player_info) {
            Ok(new_level_state) => StateTransition::Replace(Box::new(new_level_state)),
            Err(err) => {
                eprintln!("Failed to load level state \"{}\": {err}", &new_level);
                std::process::exit(1);
            }
        }
    }
}

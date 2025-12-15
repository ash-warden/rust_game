use crate::game_state::{GameStateStack, LevelState, MenuState, PlayerInfo};
use crate::player::PlayerMovementState;
use crate::resources::{RESOURCE_MANAGER, load_all_assets};
use macroquad::math::vec2;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

//module for loading the level from the map
mod game_state;
mod level;
mod player;
mod resources;
mod menu;

//convert an index to coordinates, e.g. for tile textures in a grid
fn index_to_coords(n: i32, width: i32) -> (f32, f32) {
    let x = (n - 1) % width;
    let y = (n - 1) / width;
    (x as f32, y as f32)
}

fn coords_to_index(x: i32, y: i32, width: i32) -> i32 {
    let xi = x;
    let yi = y;
    yi * width + xi
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        window_width: 640,
        window_height: 480,
        sample_count: 1,
        window_resizable: true,
        ..Default::default() // fill in the rest with defaults
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    load_all_assets().await;

    {
        let mut res = RESOURCE_MANAGER.lock().unwrap();
        res.scale = 1.;
    }

    /*let player_info = PlayerInfo {
        pos: vec2(100., 100.),
        velocity: vec2(0., 0.),
        state: PlayerMovementState::Standing,
        crouch: false,
    };

    let level_state = LevelState::build("test_2_1", player_info).unwrap_or_else(|err| {
        eprintln!("Failed to load level state: {err}");
        std::process::exit(1);
    });
    let mut game_state_stack = GameStateStack::new(Box::new(level_state));
    */

    let menu_state = MenuState::new();
    let mut game_state_stack = GameStateStack::new(Box::new(menu_state));

    build_textures_atlas();

    loop {
        game_state_stack.update();
        clear_background(BLACK);
        let scale = {
            let res = RESOURCE_MANAGER.lock().unwrap();
            res.scale
        };
        game_state_stack.draw(scale);

        let w = screen_width();
        let h = screen_height();

        if w != 640. || h != 480. {
            let new_scale = if w < 960. { 1. } else { 2. };

            {
                let mut res = RESOURCE_MANAGER.lock().unwrap();
                res.scale = new_scale;
            }

            set_window_size((640. * new_scale) as u32, (480. * new_scale) as u32);
        }

        next_frame().await;
    }
}

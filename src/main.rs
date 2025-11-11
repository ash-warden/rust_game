use crate::game_state::{GameStateStack, LevelState};
use crate::resources::load_all_assets;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

//module for loading the level from the map
mod level;
mod game_state;
mod resources;
mod player;

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
    //screen scale
    let mut scale = 1.;
    let level_state = LevelState::build().await.unwrap_or_else(|err| {
        eprintln!("Failed to load level state: {err}");
        std::process::exit(1);
    });
    let mut game_state_stack = GameStateStack::new(Box::new(level_state));

    //supposed to improve performance?
    build_textures_atlas();

    loop {
        game_state_stack.update();
        clear_background(BLACK);
        game_state_stack.draw(scale);

        //update screen size
        let w = screen_width();
        let h = screen_height();

        if w != 640. || h != 480. {
            if w < 960. {
                scale = 1.;
            } else {
                scale = 2.;
            }
            set_window_size(
                (640. * scale) as u32,
                (480. * scale) as u32,
            );
        }

        next_frame().await
    }
}

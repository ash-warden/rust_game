use crate::game_state::{GameStateStack, LevelState, MenuState, PlayerInfo};
use crate::player::PlayerMovementState;
use crate::resources::{RESOURCE_MANAGER, load_all_assets};
use macroquad::math::vec2;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

//module for loading the level from the map
mod game_state;
mod level;
mod menu;
mod player;
mod resources;

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
    /*
        let player_info = PlayerInfo {
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

    const BASE_W: f32 = 640.0;
    const BASE_H: f32 = 480.0;

    let render_target = render_target(BASE_W as u32, BASE_H as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    loop {
        // prevent screen getting too small
        let min_w = 640.0;
        let min_h = 480.0;

        let w = screen_width();
        let h = screen_height();

        if w < min_w || h < min_h {
            set_window_size(min_w as u32, min_h as u32);
        }

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            zoom: vec2(2.0 / BASE_W, 2.0 / BASE_H),
            target: vec2(BASE_W / 2.0, BASE_H / 2.0),
            ..Default::default()
        });

        game_state_stack.update();
        game_state_stack.draw(1.0);

        set_default_camera();

        clear_background(MAGENTA);

        //draw background tiles
        {
            let cols = (screen_width() / 32.).ceil() as i32;
            let rows = (screen_height() / 32.).ceil() as i32;
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("s_back_test.png");
            for y in 0..rows {
                for x in 0..cols {
                    draw_texture_ex(
                        tex,
                        x as f32 * 32.,
                        y as f32 * 32.,
                        WHITE,
                        DrawTextureParams {
                            ..Default::default()
                        },
                    );
                }
            }
        }

        let w = screen_width();
        let h = screen_height();

        let scale = ((w / BASE_W).floor().min((h / BASE_H).floor()) as i32).max(1) as f32;
        println!("{}", scale);

        let dest_w = (BASE_W * scale).round();
        let dest_h = (BASE_H * scale).round();

        let x = ((w - dest_w) / 2.0).round();
        let y = ((h - dest_h) / 2.0).round();

        draw_texture_ex(
            &render_target.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

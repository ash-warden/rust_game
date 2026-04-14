use crate::cutscene::CutsceneState;
use crate::game_state::{GameStateStack, LoadSaveState, MenuState, NewGameState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::resources::{Resources, get_text, load_all_assets};
use macroquad::math::vec2;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
mod area;
mod controls;
mod current_game;
mod cutscene;
mod game_state;
mod hud;
mod level;
mod level_state;
mod menu;
mod obj_checkpoint;
mod obj_door;
mod obj_npc;
mod obj_star;
mod player;
mod player_functions;
mod resources;
mod room_obj_from_file;
mod text;
mod traits_for_obj;

const SCREEN_SIZE: IVec2 = ivec2(640, 480);
const SCREEN_SIZE_F: Vec2 = vec2(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32);
const SCREEN_SIZE_U: UVec2 = uvec2(SCREEN_SIZE.x as u32, SCREEN_SIZE.y as u32);

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        window_width: SCREEN_SIZE.x,
        window_height: SCREEN_SIZE.y,
        sample_count: 1,
        window_resizable: true,
        ..Default::default() // fill in the rest with defaults
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    load_all_assets().await;

    let new_game_text = get_text("new_game");
    let load_game_text = get_text("load_file");

    let menu_pos = menu_centre_pos(11, 3);
    let mut menu = Menu::new(menu_pos.x, menu_pos.y);
    let title = MenuItem::new("game_25", || StateTransition::None, false);
    menu.add_item(title);
    let start_game_new = MenuItem::new(
        &new_game_text,
        move || StateTransition::Push(Box::new(NewGameState::new().clone())),
        true,
    );
    menu.add_item(start_game_new);
    let start_game_load = MenuItem::new(
        &load_game_text,
        move || StateTransition::Push(Box::new(LoadSaveState::new())),
        true,
    );
    menu.add_item(start_game_load);
    let test_cutscene = MenuItem::new(
        "Cutscene test",
        move || StateTransition::Push(Box::new(CutsceneState::new("intro"))),
        true,
    );
    menu.add_item(test_cutscene);

    let menu_state = MenuState::new(menu);

    let mut game_state_stack = GameStateStack::new(Box::new(menu_state));

    build_textures_atlas();

    let render_target = render_target(SCREEN_SIZE_U.x, SCREEN_SIZE_U.y);
    render_target.texture.set_filter(FilterMode::Nearest);

    loop {
        // prevent screen getting too small
        let w = screen_width();
        let h = screen_height();

        if w < SCREEN_SIZE_F.x || h < SCREEN_SIZE_F.y {
            set_window_size(SCREEN_SIZE_U.x, SCREEN_SIZE_U.y);
        }

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            zoom: vec2(2.0 / SCREEN_SIZE_F.x, 2.0 / SCREEN_SIZE_F.y),
            target: vec2(SCREEN_SIZE_F.x / 2.0, SCREEN_SIZE_F.y / 2.0),
            ..Default::default()
        });

        clear_background(GRAY);

        game_state_stack.update();
        game_state_stack.draw();

        set_default_camera();

        //draw background tiles
        {
            let cols = (screen_width() / 32.).ceil() as i32;
            let rows = (screen_height() / 32.).ceil() as i32;
            let res = Resources::global();
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

        let scale = ((w / SCREEN_SIZE_F.x)
            .floor()
            .min((h / SCREEN_SIZE_F.y).floor()) as i32)
            .max(1) as f32;

        let dest_w = (SCREEN_SIZE_F.x * scale).round();
        let dest_h = (SCREEN_SIZE_F.y * scale).round();

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

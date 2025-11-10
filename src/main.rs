use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

//module for loading the level from the map
mod level;

//convert an index to coordinates, e.g. for tile textures in a grid
fn index_to_coords(n: i32, width: i32) -> (f32, f32) {
    let x = (n - 1) % width;
    let y = (n - 1) / width;
    (x as f32, y as f32)
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
    //screen scale
    let mut scale = 1.;

    let level = level::Level::build("test_map1.json").await.unwrap();

    //supposed to improve performance?
    build_textures_atlas();

    loop {
        //draw tiles
        clear_background(BLACK);
        let mut i = 0; //tile number
        let mut x = 0.; //x coord
        let mut y = 0.; //y coord
        while y < 15. {
            while x < 20. {
                draw_texture_ex(
                    &level.tile_image,
                    x * 32. * scale,
                    y * 32. * scale,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32. * scale, 32. * scale)),
                        source: Some(Rect::new(
                            index_to_coords(level.tile_values[i], 2).0*32.,
                            index_to_coords(level.tile_values[i], 2).1*32.,
                            32.,
                            32.,
                        )),
                        ..Default::default()
                    },
                );
                x += 1.;
                i += 1;
            }
            x = 0.;
            y += 1.;
        }
        //update screen size
        let w = screen_width();
        let h = screen_height();
        if w != 640. || h != 480. {
            if w < 960. {
                set_window_size(640, 480);
                scale = 1.;
            } else {
                set_window_size(1280, 960);
                scale = 2.;
            }
        }

        next_frame().await
    }
}

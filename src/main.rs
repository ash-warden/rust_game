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
    let level = level::Level::build("assets/maps/test_map1.json").await
        .unwrap_or_else(|err| {
            eprintln!("Failed to load level: {err}");
            std::process::exit(1);
        });

    //supposed to improve performance?
    build_textures_atlas();

    loop {
        //draw tiles
        clear_background(BLACK);
        let mut i = 0; //tile number
        let mut x = 0.; //x coord
        let mut y = 0.; //y coord

        let map_width = level.map_dimensions.0;
        let map_height = level.map_dimensions.1;

        while y < map_height {
            //column
            while x < map_width {
                //row
                draw_texture_ex(
                    &level.tile_image,
                    x * level.tile_size * scale,
                    y * level.tile_size * scale,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(level.tile_size * scale, level.tile_size * scale)),
                        source: Some(Rect::new(
                            index_to_coords(level.tile_values[i], level.tileset_columns).0
                                * level.tile_size,
                            index_to_coords(level.tile_values[i], level.tileset_columns).1
                                * level.tile_size,
                            level.tile_size,
                            level.tile_size,
                        )),
                        ..Default::default()
                    },
                );
                x += 1.;
                i += 1;
                if i > level.tile_values.len() {
                    println!("too many tiles to draw!");
                    break;
                }
            }
            x = 0.; // go back to beginning of row
            y += 1.;
        }
        //update screen size
        let w = screen_width();
        let h = screen_height();
        if w != level.map_dimensions.0 * level.tile_size
            || h != level.map_dimensions.1 * level.tile_size
        {
            if w < level.map_dimensions.0 * level.tile_size * 2. {
                scale = 1.;
            } else {
                scale = 2.;
            }
            set_window_size(
                (level.map_dimensions.0 * level.tile_size * scale) as u32,
                (level.map_dimensions.1 * level.tile_size * scale) as u32,
            );
        }

        next_frame().await
    }
}

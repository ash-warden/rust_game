// IF DIMENSIONS OF ROOM OR SCREEN CHANGE A TON HERE NEEDS TO BE CHANGED!!!
use std::cmp::min;

use macroquad::{
    color::{Color, WHITE},
    math::{Vec2, ivec2, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use crate::{
    SCREEN_SIZE,
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    resources::Resources,
    text::write_text,
};

fn draw_health_bar(pos: Vec2, transparency: f32) {
    //health
    let thickness = 16.;
    let width_single_section = 10.;
    let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
    let health = cur_game.health;
    //meter background
    draw_rectangle_lines(
        pos.x - 1.,
        pos.y - 1.,
        MAX_HEALTH as f32 * width_single_section + 2.,
        thickness + 2.,
        2.,
        Color::new(0., 0., 0., transparency),
    );
    draw_rectangle(
        pos.x + width_single_section * health as f32,
        pos.y,
        (MAX_HEALTH - min(health, MAX_HEALTH)) as f32 * width_single_section,
        thickness,
        Color::new(0., 0., 0., transparency),
    );
    //meter
    draw_rectangle(
        pos.x,
        pos.y,
        width_single_section * health as f32,
        thickness,
        Color::new(1., 1., 1., transparency),
    );
}

fn draw_health_text(pos: Vec2, transparency: f32) {
    // label
    write_text(
        "HEALTH",
        vec2(pos.x, pos.y + 16.),
        Color::new(0.3, 0.3, 0.3, transparency),
        "font_bold.png",
    );
}

#[derive(Clone, Debug)]
pub enum MapPixelType {
    Solid,
    Ladder,
    Air,
}

pub fn get_map_pixels(cur_area: &str, cur_room: &str) -> Vec<Vec<MapPixelType>> {
    let cur_room_ints = ivec2(
        cur_room.split_once("_").unwrap().0.parse::<i32>().unwrap(),
        cur_room.split_once("_").unwrap().1.parse::<i32>().unwrap(),
    );
    let mut map_pixels: Vec<Vec<MapPixelType>> = vec![];
    {
        let res = Resources::global();
        for y in ((cur_room_ints.y - 1)..=(cur_room_ints.y + 1)).rev() {
            for x in (cur_room_ints.x - 1)..=(cur_room_ints.x + 1) {
                map_pixels.push(vec![]);
                let room_str = format!("{}_{}_{}", cur_area, x, y);
                if let Some(room) = res.get_room(&room_str) {
                    for i in 0..room.map_dimensions.y {
                        for j in 0..room.map_dimensions.x {
                            let tile = room.get_tile_info(ivec2(j, i));
                            if tile.ladder {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Ladder);
                            } else if tile.solid {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Solid);
                            } else {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Air);
                            }
                        }
                    }
                } else {
                    for _ in 0..(20 * 15) {
                        map_pixels.last_mut().unwrap().push(MapPixelType::Solid);
                    }
                }
            }
        }
    }
    return map_pixels;
    //return vec![MapPixelType::Solid]; // temporary
}

fn draw_hud_map(pos: Vec2, transparency: f32, map_pixels: &Vec<Vec<MapPixelType>>) {
    let map_dim = ivec2(20, 15);
    let no_rooms_h = 3;
    let map_scale = 4;
    let mut room_x = 0;
    let mut room_y = 0;
    for room in map_pixels {
        let mut tile_x = 0;
        let mut tile_y = 0;
        for tile in room {
            let color: Color;
            // current room
            if room_x == 1 && room_y == 1 {
                match tile {
                    MapPixelType::Solid => color = Color::new(0.2, 0.2, 0.2, transparency),
                    MapPixelType::Ladder => color = Color::new(0.3, 0.7, 0.7, transparency),
                    MapPixelType::Air => color = Color::new(1., 1., 1., transparency),
                }
                // other rooms
            } else {
                match tile {
                    MapPixelType::Solid => color = Color::new(0., 0., 0., transparency),
                    MapPixelType::Ladder => color = Color::new(0.3, 0.7, 0.7, transparency),
                    MapPixelType::Air => color = Color::new(0.6, 0.6, 0.6, transparency),
                }
            }

            draw_rectangle(
                ((room_x * map_dim.x + tile_x) * map_scale) as f32
                    + pos.x
                    + (SCREEN_SIZE.x - (map_dim.x * no_rooms_h * map_scale) - 24 - 24) as f32,
                ((room_y * map_dim.y + tile_y) * map_scale) as f32 + pos.y,
                map_scale as f32,
                map_scale as f32,
                color,
            );
            tile_x += 1;
            if tile_x >= map_dim.x {
                tile_x = 0;
                tile_y += 1;
            }
        }
        room_x += 1;
        if room_x >= no_rooms_h {
            room_x = 0;
            room_y += 1;
        }
    }
}

fn draw_hint_box(pos: Vec2, text: &str) {
    draw_rectangle(pos.x, pos.y, 200., 96., Color::new(0.1, 0.1, 0.1, 0.5));
    write_text(text, vec2(pos.x + 6., pos.y), WHITE, "font.png");
}

pub fn draw_hud(bottom: bool, full: bool, map_pixels: &Vec<Vec<MapPixelType>>, hint_text: &str) {
    let health_pos = if bottom {
        vec2(24., 408.) // 480 − 24 − 32 − 16
    } else {
        vec2(24., 25.)
    };
    let map_pos = if bottom {
        vec2(24., 276.) // 480 − (4 × 45) − 24
    } else {
        vec2(24., 24.)
    };
    let text_pos = if bottom {
        vec2(152., 360.) // 480 − 96 − 24
    } else {
        vec2(152., 24.)
    };

    let transparency = if full { 0.8 } else { 0.6 };
    draw_health_bar(health_pos, transparency);
    if full {
        draw_health_text(health_pos, transparency);
        draw_hud_map(map_pos, transparency, map_pixels);
    }
    if !hint_text.is_empty() {
        draw_hint_box(text_pos, hint_text);
    }
}

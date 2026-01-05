use std::cmp::min;

use macroquad::{
    color::{BLACK, BLUE, Color, WHITE},
    math::{Vec2, ivec2, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

use crate::{
    current_game::{CURRENT_GAME_MANAGER, MAX_HEALTH},
    resources::RESOURCE_MANAGER,
    text::write_text,
};

fn draw_health_bar(pos: Vec2, transparency: f32) {
    //health
    let thickness = 16.;
    let cur_game = CURRENT_GAME_MANAGER.lock().unwrap();
    let health = cur_game.health;
    //meter background
    draw_rectangle_lines(
        pos.x - 1.,
        pos.y - 1.,
        MAX_HEALTH as f32 * 10. + 2.,
        thickness + 2.,
        2.,
        Color::new(0., 0., 0., transparency),
    );
    draw_rectangle(
        pos.x + 10. * health as f32,
        pos.y,
        (MAX_HEALTH - min(health, MAX_HEALTH)) as f32 * 10.,
        thickness,
        Color::new(1., 0.3, 0.3, transparency),
    );
    //meter
    draw_rectangle(
        pos.x,
        pos.y,
        10. * health as f32,
        thickness,
        Color::new(0.3, 0.3, 1., transparency),
    );
}

fn draw_health_text(pos: Vec2, transparency: f32) {
    // label
    write_text(
        "HEALTH",
        vec2(pos.x, pos.y + 16.),
        Color::new(1., 0.3, 0.3, transparency),
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
    println!("hud map area and room {} {}", cur_area, cur_room);
    let cur_room_ints = ivec2(
        cur_room.split_once("_").unwrap().0.parse::<i32>().unwrap(),
        cur_room.split_once("_").unwrap().1.parse::<i32>().unwrap(),
    );
    let mut map_pixels: Vec<Vec<MapPixelType>> = vec![];
    {
        let res = RESOURCE_MANAGER.lock().unwrap();
        for y in ((cur_room_ints.y - 1)..=(cur_room_ints.y + 1)).rev() {
            for x in (cur_room_ints.x - 1)..=(cur_room_ints.x + 1) {
                map_pixels.push(vec![]);
                let room_str = format!("{}_{}_{}", cur_area, x, y);
                println!("{}", room_str);
                if let Some(room) = res.get_room(&room_str) {
                    for i in 0..room.map_dimensions.y {
                        for j in 0..room.map_dimensions.x {
                            println!("{:?} {:?}", i, j);
                            let tile = room.get_tile_info(ivec2(j, i));
                            if tile.solid {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Solid);
                            } else if tile.ladder {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Ladder);
                            } else {
                                map_pixels.last_mut().unwrap().push(MapPixelType::Air);
                            }
                        }
                    }
                } else {
                    for _ in 0..(20 * 15) {
                        map_pixels.last_mut().unwrap().push(MapPixelType::Air);
                    }
                }
            }
        }
    }
    println!("{:?}", map_pixels);
    return map_pixels;
    //return vec![MapPixelType::Solid]; // temporary
}

fn draw_hud_map(map_pixels: &Vec<Vec<MapPixelType>>) {
    let mut room_x = 0;
    let mut room_y = 0;
    for room in map_pixels {
        let mut tile_x = 0;
        let mut tile_y = 0;
        for tile in room {
            let color: Color;
            match tile {
                MapPixelType::Solid => color = WHITE,
                MapPixelType::Ladder => color = BLUE,
                MapPixelType::Air => color = BLACK,
            }
            draw_rectangle(
                (room_x + tile_x) as f32,
                (room_y + tile_y) as f32,
                2.,
                2.,
                color,
            );
            tile_x += 2;
            if tile_x >= 40 {
                tile_x = 0;
                tile_y += 2;
            }
        }
        room_x += 40;
        if room_x >= 120 {
            room_x = 0;
            room_y += 30;
        }
    }
}

pub fn draw_hud(bottom: bool, solid: bool, map_pixels: &Vec<Vec<MapPixelType>>) {
    let pos = if bottom {
        vec2(24., 324.) // change again once map done
    } else {
        vec2(24., 24.)
    };
    let transparency = if solid { 1.0 } else { 0.3 };
    draw_health_bar(pos, transparency);
    if solid {
        draw_health_text(pos, transparency);
    }
    draw_hud_map(map_pixels);
}

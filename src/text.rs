use macroquad::{
    color::Color,
    math::{Rect, Vec2, vec2},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

use crate::resources::RESOURCE_MANAGER;

pub fn write_text(text: &str, pos: Vec2, color: Color) {
    let mut row = 0.;
    let mut col = 0.;
    for item in text.chars() {
        let letter_ascii = item as u8;
        //println!("{}", letter_ascii);

        if letter_ascii == 10 {
            row += 1.;
            col = 0.;
        } else {
            let letter_pos = Vec2 {
                x: col * 16.,
                y: row * 32.,
            };
            {
                let res = RESOURCE_MANAGER.lock().unwrap();
                let tex: &Texture2D;
                tex = res.get_texture("font.png");
                draw_texture_ex(
                    tex,
                    letter_pos.x + pos.x,
                    letter_pos.y + pos.y,
                    color,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 32.)),
                        source: Some(Rect::new(
                            16. * (letter_ascii % 32) as f32,
                            32. * (letter_ascii / 32) as f32 - 32.,
                            16.,
                            32.,
                        )),
                        ..Default::default()
                    },
                );
            }
            col += 1.;
        }
    }
}

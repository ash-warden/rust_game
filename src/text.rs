use macroquad::{
    color::Color,
    math::{Rect, Vec2, vec2},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

use crate::resources::Resources;

pub fn write_text(text: &str, pos: Vec2, color: Color, font: &str) {
    let letter_size = vec2(16., 32.);
    let mut row = 0.;
    let mut col = 0.;
    for item in text.chars() {
        let letter_ascii = item as u8;
        //this is a \n line break
        if letter_ascii == 10 {
            row += 1.;
            col = 0.;
        } else {
            let letter_pos = Vec2 {
                x: col * letter_size.x,
                y: row * letter_size.y,
            };
            {
                let res = Resources::global();
                let tex: &Texture2D;
                tex = res.get_texture(font);
                let no_letters_in_row = 32;
                draw_texture_ex(
                    tex,
                    letter_pos.x + pos.x,
                    letter_pos.y + pos.y,
                    color,
                    DrawTextureParams {
                        dest_size: Some(vec2(letter_size.x, letter_size.y)),
                        source: Some(Rect::new(
                            letter_size.x * (letter_ascii % no_letters_in_row) as f32,
                            letter_size.y * (letter_ascii / no_letters_in_row) as f32 - 32.,
                            letter_size.x,
                            letter_size.y,
                        )),
                        ..Default::default()
                    },
                );
            }
            col += 1.;
        }
    }
}

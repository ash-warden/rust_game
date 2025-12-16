use crate::index_to_coords;
use crate::resources::RESOURCE_MANAGER;
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, Texture2D, draw_texture_ex};
use macroquad::text::{TextParams, draw_text, draw_text_ex};

pub struct Menu {
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        let mut menu_items = Vec::new();
        let test_item = MenuItem::new("Item1012\n34");
        menu_items.push(test_item);
        Menu { menu_items }
    }

    pub fn draw(&self, scale: f32) {
        {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("background.png");
            draw_texture_ex(
                tex,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(48. * scale, 48. * scale)),
                    source: None,
                    ..Default::default()
                },
            );
        }
        for item in &self.menu_items {
            item.draw(scale);
        }
    }
}

pub struct MenuItem {
    label_text: String,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        MenuItem {
            label_text: label.to_string(),
        }
    }

    pub fn draw(&self, scale: f32) {
        let mut row = 0.;
        let mut col = 0.;
        for (index, item) in self.label_text.chars().enumerate() {
            let letter_ascii: i32 = item.to_ascii_lowercase() as i32;
            //println!("{}", letter_ascii);

            if letter_ascii == 10 {
                row += 1.;
                col = 0.;
            } else {
                let letter_pos = Vec2 {
                    x: col * 19. + 30.,
                    y: row * 38. + 30.,
                } * scale;
                {
                    let res = RESOURCE_MANAGER.lock().unwrap();
                    let tex = res.get_texture("font.png");
                    draw_texture_ex(
                        tex,
                        letter_pos.x,
                        letter_pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(19. * scale, 38. * scale)),
                            source: Some(Rect::new(
                                19. * (letter_ascii % 32) as f32,
                                38. * (letter_ascii / 32) as f32 - 38.,
                                19.,
                                38.,
                            )),
                            ..Default::default()
                        },
                    );
                }
                col += 1.;
            }
        }
    }
}

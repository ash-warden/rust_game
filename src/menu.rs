use macroquad::color::{WHITE};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::text::{draw_text, draw_text_ex, TextParams};
use crate::index_to_coords;
use crate::resources::RESOURCE_MANAGER;

pub struct Menu {
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        let mut menu_items = Vec::new();
        let test_item = MenuItem::new("Item101234");
        menu_items.push(test_item);
        Menu { menu_items }
    }

    pub fn draw(&self, scale: f32) {
        {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("background.png");
            draw_texture_ex(
                tex,
                0., 0.,
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
        MenuItem{ label_text: label.to_string() }
    }

    pub fn draw(&self, scale: f32) {
        let mut row = 0.;
        let mut col = 0.;
        for (index, item) in self.label_text.chars().enumerate() {
            let letter_ascii: i32 = item.to_ascii_lowercase() as i32;
            //println!("{}", letter);
            // todo handle line breaks
            let letter_pos = Vec2{x: col * 19. + 30., y: row * 38. + 30.};
            {
                let res = RESOURCE_MANAGER.lock().unwrap();
                let tex = res.get_texture("font.png");
                draw_texture_ex(
                    tex,
                    letter_pos.x, letter_pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: None,
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
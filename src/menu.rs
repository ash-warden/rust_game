use crate::game_state::StateTransition;
use crate::index_to_coords;
use crate::resources::RESOURCE_MANAGER;
use macroquad::color::{Color, WHITE, YELLOW};
use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, Texture2D, draw_texture_ex};
use macroquad::text::{TextParams, draw_text, draw_text_ex};

pub struct Menu {
    menu_items: Vec<MenuItem>,
    current_index: u32,
}

impl Menu {
    pub fn new() -> Self {
        let menu_items = Vec::new();
        Menu {
            menu_items,
            current_index: 0,
        }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.menu_items.push(item);
    }

    pub fn update(&mut self) -> StateTransition {
        if is_key_pressed(KeyCode::Down) {
            self.current_index += 1;
        }
        if is_key_pressed(KeyCode::Up) {
            self.current_index -= 1;
        }
        if is_key_pressed(KeyCode::Enter) {
            return self.menu_items[self.current_index as usize].activate();
        }
        StateTransition::None
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
        let mut i = 0.;
        for item in &self.menu_items {
            let mut selected = false;
            if self.current_index == i as u32 {
                selected = true;
            }
            item.draw(scale, vec2(0., i), selected);
            i += 1.;
        }
    }
}

pub struct MenuItem {
    label_text: String,
    action: Box<dyn FnMut() -> StateTransition>,
}

impl MenuItem {
    pub fn new<F>(label: &str, action: F) -> Self
    where
        F: FnMut() -> StateTransition + 'static,
    {
        MenuItem {
            label_text: label.to_string(),
            action: Box::new(action),
        }
    }

    pub fn activate(&mut self) -> StateTransition {
        (self.action)()
    }

    pub fn draw(&self, scale: f32, offset: Vec2, selected: bool) {
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
                    let mut color: Color = WHITE;
                    if selected {
                        color = YELLOW;
                    }
                    draw_texture_ex(
                        tex,
                        letter_pos.x + offset.x,
                        letter_pos.y + offset.y * 38.,
                        color,
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

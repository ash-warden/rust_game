use crate::game_state::StateTransition;
use crate::resources::RESOURCE_MANAGER;
use macroquad::color::{Color, WHITE, YELLOW};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex};
use macroquad::texture::Texture2D;
use crate::controls::CONTROLS;

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
        //check to avoid highlighting the title etc
        if !self.menu_items[self.current_index as usize].selectable {
            self.current_index += 1;
        }
        let mut input = CONTROLS.lock().unwrap();
        if input.controls_down() {
            if self.current_index < self.menu_items.len() as u32 - 1 {
                self.current_index += 1;
                if !self.menu_items[self.current_index as usize].selectable {
                    if self.current_index < self.menu_items.len() as u32 - 1 {
                        self.current_index += 1;
                    } else { self.current_index -= 1 }
                }
            }
        }
        if input.controls_up() {
            if self.current_index > 0 {
                self.current_index -= 1;
                if !self.menu_items[self.current_index as usize].selectable {
                    if self.current_index > 0 {
                        self.current_index -= 1;
                    } else { self.current_index += 1 }
                }
            }
        }
        if input.controls_enter() {
            return self.menu_items[self.current_index as usize].activate();
        }
        StateTransition::None
    }

    pub fn draw(&self) {
        let mut menu_width: i32 = 0;
        let menu_height: i32 = *&self.menu_items.len() as i32;
        for item in &self.menu_items {
            if item.label_text.len() as i32 > menu_width {
                menu_width = item.label_text.len() as i32;
            }
        }
        {
            let res = RESOURCE_MANAGER.lock().unwrap();
            let tex = res.get_texture("background.png");
            //corners
            //top left
            draw_texture_ex(
                tex,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(16., 16.)),
                    source: Some(Rect::new(0., 0., 16., 16.)),
                    ..Default::default()
                },
            );
            //top right
            draw_texture_ex(
                tex,
                menu_width as f32 * 16. + 16.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(16., 16.)),
                    source: Some(Rect::new(32., 0., 16., 16.)),
                    ..Default::default()
                },
            );
            //bottom left
            draw_texture_ex(
                tex,
                0.,
                menu_height as f32 * 32. + 16.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(16., 16.)),
                    source: Some(Rect::new(0., 32., 16., 16.)),
                    ..Default::default()
                },
            );
            //bottom right
            draw_texture_ex(
                tex,
                menu_width as f32 * 16. + 16.,
                menu_height as f32 * 32. + 16.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(16., 16.)),
                    source: Some(Rect::new(32., 32., 16., 16.)),
                    ..Default::default()
                },
            );
            //edges
            //top
            for i in 0..menu_width {
                draw_texture_ex(
                    tex,
                    i as f32 * 16. + 16.,
                    0.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(16., 0., 16., 16.)),
                        ..Default::default()
                    },
                );
            }
            //bottom
            for i in 0..menu_width {
                draw_texture_ex(
                    tex,
                    i as f32 * 16. + 16.,
                    menu_height as f32 * 32. + 16.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(16., 32., 16., 16.)),
                        ..Default::default()
                    },
                );
            }
            // left edge
            for j in 0..menu_height*2 {
                draw_texture_ex(
                    tex,
                    0.,
                    j as f32 * 16. + 16.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(0., 16., 16., 16.)),
                        ..Default::default()
                    },
                );
            }

            // right edge
            for j in 0..menu_height*2 {
                draw_texture_ex(
                    tex,
                    menu_width as f32 * 16. + 16.,
                    j as f32 * 16. + 16.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(32., 16., 16., 16.)),
                        ..Default::default()
                    },
                );
            }

            // filling (center tiles)
            for i in 0..menu_width {
                for j in 0..menu_height * 2 {
                    draw_texture_ex(
                        tex,
                        i as f32 * 16. + 16.,
                        j as f32 * 16. + 16.,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(16., 16.)),
                            source: Some(Rect::new(16., 16., 16., 16.)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        let mut i = 0.;
        for item in &self.menu_items {
            let mut selected = false;
            if self.current_index == i as u32 {
                selected = true;
            }
            item.draw(vec2(16., i), selected);
            i += 1.;
        }
    }
}

pub struct MenuItem {
    label_text: String,
    action: Box<dyn FnMut() -> StateTransition>,
    selectable: bool,
}

impl MenuItem {
    pub fn new<F>(label: &str, action: F, selectable: bool) -> Self
    where
        F: FnMut() -> StateTransition + 'static,
    {
        MenuItem {
            label_text: label.to_string(),
            action: Box::new(action),
            selectable,
        }
    }

    pub fn activate(&mut self) -> StateTransition {
        (self.action)()
    }

    pub fn draw(&self, offset: Vec2, selected: bool) {
        let mut row = 0.;
        let mut col = 0.;
        for item in self.label_text.chars() {
            let letter_ascii: i32 = item.to_ascii_lowercase() as i32;
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
                    let tex :&Texture2D;
                    if self.selectable {
                        tex = res.get_texture("font.png");
                    } else {
                        tex = res.get_texture("font_non_selectable.png");
                    }
                    let mut color: Color = WHITE;
                    if selected {
                        color = YELLOW;
                    }
                    draw_texture_ex(
                        tex,
                        letter_pos.x + offset.x,
                        letter_pos.y + offset.y * 32. + 16.,
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
}

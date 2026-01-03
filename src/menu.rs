use crate::controls::CONTROLS;
use crate::game_state::StateTransition;
use crate::resources::RESOURCE_MANAGER;
use crate::text::write_text;
use macroquad::color::{BLUE, Color, WHITE, YELLOW};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex};
use macroquad::texture::Texture2D;

pub struct Menu {
    menu_items: Vec<MenuItem>,
    current_index: u32,
    pos: Vec2,
}

impl Menu {
    pub fn new(x: f32, y: f32) -> Self {
        let menu_items = Vec::new();
        Menu {
            menu_items,
            current_index: 0,
            pos: vec2(x, y),
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
                    } else {
                        self.current_index -= 1
                    }
                }
            }
        }
        if input.controls_up() {
            if self.current_index > 0 {
                self.current_index -= 1;
                if !self.menu_items[self.current_index as usize].selectable {
                    if self.current_index > 0 {
                        self.current_index -= 1;
                    } else {
                        self.current_index += 1
                    }
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
                self.pos.x,
                self.pos.y,
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
                self.pos.x + menu_width as f32 * 16. + 16.,
                self.pos.y,
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
                self.pos.x,
                self.pos.y + menu_height as f32 * 32. + 16.,
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
                self.pos.x + menu_width as f32 * 16. + 16.,
                self.pos.y + menu_height as f32 * 32. + 16.,
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
                    self.pos.x + i as f32 * 16. + 16.,
                    self.pos.y,
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
                    self.pos.x + i as f32 * 16. + 16.,
                    self.pos.y + menu_height as f32 * 32. + 16.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(16., 32., 16., 16.)),
                        ..Default::default()
                    },
                );
            }
            // left edge
            for j in 0..menu_height * 2 {
                draw_texture_ex(
                    tex,
                    self.pos.x,
                    self.pos.y + j as f32 * 16. + 16.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16., 16.)),
                        source: Some(Rect::new(0., 16., 16., 16.)),
                        ..Default::default()
                    },
                );
            }

            // right edge
            for j in 0..menu_height * 2 {
                draw_texture_ex(
                    tex,
                    self.pos.x + menu_width as f32 * 16. + 16.,
                    self.pos.y + j as f32 * 16. + 16.,
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
                        self.pos.x + i as f32 * 16. + 16.,
                        self.pos.y + j as f32 * 16. + 16.,
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
            item.draw(vec2(self.pos.x + 16., self.pos.y + 16.), i, selected);
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

    pub fn draw(&self, offset: Vec2, line: f32, selected: bool) {
        let pos = vec2(offset.x, offset.y + line * 32.);
        let color: Color;
        if !self.selectable {
            color = BLUE;
        } else if selected {
            color = YELLOW;
        } else {
            color = WHITE;
        }
        write_text(&self.label_text, pos, color);
    }
}

pub fn menu_centre_pos(char_w: i32, char_h: i32) -> Vec2 {
    let centre_w = 640. / 2. - (char_w * 16 + 32) as f32 / 2.;
    let centre_h = 480. / 2. - (char_h * 32 + 32) as f32 / 2.;
    vec2(centre_w, centre_h)
}

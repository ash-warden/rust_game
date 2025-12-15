use macroquad::color::{WHITE};
use macroquad::text::{draw_text, draw_text_ex, TextParams};
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
        for item in &self.menu_items {
            item.draw(scale);
        }
    }
}

pub struct MenuItem {
    label: String,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        MenuItem{label: label.to_string() }
    }

    pub fn draw(&self, scale: f32) {
        let res = RESOURCE_MANAGER.lock().unwrap();
        draw_text_ex(
            &self.label,
            20.0,
            40.0,
            TextParams {
                font: Some(res.get_font()),
                font_size: (16.0 * scale) as u16,
                color: WHITE,
                ..Default::default()
            }
        );
    }
}
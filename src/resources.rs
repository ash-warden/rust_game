use crate::level::Level;
use macroquad::prelude::Texture2D;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use macroquad::text::{load_ttf_font, Font};
use walkdir::WalkDir;

pub struct Resources {
    pub levels: HashMap<String, Arc<Level>>,
    pub textures: HashMap<String, Texture2D>,
    pub scale: f32,
    pub font: Option<Font>
}

impl Resources {
    pub fn new() -> Self {
        Self {
            levels: HashMap::new(),
            textures: HashMap::new(),
            scale: 1.,
            font: None,
        }
    }

    pub fn get_texture(&self, key: &str) -> &Texture2D {
        match self.textures.get(key) {
            Some(tex) => tex,
            None => {
                println!("Warning: texture '{}' not found, using fallback", key);
                self.textures
                    .get("missing")
                    .expect("Missing texture not loaded")
            }
        }
    }

    pub fn get_level(&self, key: &str) -> Option<Arc<Level>> {
        self.levels.get(key).cloned()
    }

    pub fn insert_texture(&mut self, key: String, texture: Texture2D) {
        self.textures.insert(key, texture);
    }

    pub fn insert_level(&mut self, key: String, level: Arc<Level>) {
        self.levels.insert(key, level);
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = Some(font);
    }
    pub fn get_font(&self) -> &Font {self.font.as_ref().unwrap()}
}

pub static RESOURCE_MANAGER: Lazy<Mutex<Resources>> = Lazy::new(|| Mutex::new(Resources::new()));

pub async fn load_all_assets() {
    let mut res = RESOURCE_MANAGER.lock().unwrap();
    //load font
    res.set_font(load_ttf_font("assets/Hack-Regular.ttf")
        .await
        .unwrap());

    for entry in WalkDir::new("assets") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let key = path.file_stem().unwrap().to_str().unwrap();
            let path_str = path.to_str().unwrap();
            println!("Loading asset: '{}'", path_str);

            match ext {
                "png" => {
                    let texture = macroquad::texture::load_texture(path_str).await.unwrap();
                    res.insert_texture(key.to_string() + ".png", texture);
                }
                "tmj" => {
                    let level = Level::build(path_str).await.unwrap();
                    res.insert_level(key.to_string(), Arc::new(level));
                }
                _ => {
                    println!("Skipping unsupported file: {}", path_str);
                }
            }
        }
    }
    let missing_texture = macroquad::texture::load_texture("assets/missing.png")
        .await
        .unwrap();
    missing_texture.set_filter(macroquad::texture::FilterMode::Nearest);
    res.insert_texture("missing".to_string(), missing_texture);
}

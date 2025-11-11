use crate::level;
use macroquad::prelude::Texture2D;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use crate::level::Level;

pub struct Resources {
    pub levels: HashMap<String, Arc<level::Level>>,
    pub textures: HashMap<String, Texture2D>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            levels: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(&self, key: &str) -> Option<&Texture2D> {
        self.textures.get(key)
    }

    pub fn get_level(&self, key: &str) -> Option<Arc<Level>> {
        self.levels.get(key).cloned()
    }

    pub fn insert_texture(&mut self, key: String, texture: Texture2D) {
        self.textures.insert(key, texture);
    }

    pub fn insert_level(&mut self, key: String, level: Arc<level::Level>) {
        self.levels.insert(key, level);
    }
}

pub static RESOURCE_MANAGER: Lazy<Mutex<Resources>> = Lazy::new(|| Mutex::new(Resources::new()));

pub async fn load_all_assets() {
    let mut res = RESOURCE_MANAGER.lock().unwrap();
    for entry in WalkDir::new("assets/maps") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let key = path.file_stem().unwrap().to_str().unwrap();
            let path_str = path.to_str().unwrap();

            match ext {
                "png" => {
                    let texture = macroquad::texture::load_texture(path_str).await.unwrap();
                    res.insert_texture(key.to_string() + ".png", texture);
                }
                "tmj" => {
                    let level = level::Level::build(path_str).await.unwrap();
                    res.insert_level(key.to_string(), Arc::new(level));
                }
                _ => {
                    println!("Skipping unsupported file: {}", path_str);
                }
            }
        }
    }
}
use crate::level::Room;
use crate::room_obj_from_file::RoomObjectsFromFile;
use crate::traits_for_obj::Obj;
use macroquad::prelude::Texture2D;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

pub struct Resources {
    pub rooms: HashMap<String, Arc<Room>>,
    pub room_objects: HashMap<String, Arc<RoomObjects>>,
    pub textures: HashMap<String, Texture2D>,
    pub scale: f32,
    pub background_texture: Option<String>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            room_objects: HashMap::new(),
            textures: HashMap::new(),
            scale: 1.,
            background_texture: None,
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

    pub fn get_room(&self, key: &str) -> Option<Arc<Room>> {
        if self.rooms.contains_key(key) {
            self.rooms.get(key).cloned()
        } else {
            None
        }
    }

    pub fn get_room_object(&self, key: &str) -> Option<Arc<RoomObjects>> {
        self.room_objects.get(key).cloned()
    }

    pub fn insert_texture(&mut self, key: String, texture: Texture2D) {
        self.textures.insert(key, texture);
    }

    pub fn insert_room(&mut self, key: String, level: Arc<Room>) {
        self.rooms.insert(key, level);
    }

    pub fn insert_object(&mut self, key: String, object: Arc<RoomObjects>) {
        self.room_objects.insert(key, object);
    }
}

pub static RESOURCE_MANAGER: Lazy<Mutex<Resources>> = Lazy::new(|| Mutex::new(Resources::new()));

pub async fn load_all_assets() {
    let mut res = RESOURCE_MANAGER.lock().unwrap();

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
                    texture.set_filter(macroquad::texture::FilterMode::Nearest);
                    res.insert_texture(key.to_string() + ".png", texture);
                }
                "tmj" => {
                    let level = Room::build(path_str).await.unwrap();
                    res.insert_room(key.to_string(), Arc::new(level));
                    println!("{}", key);
                }
                "roj" => {
                    //"room object json". One file per area
                    let objects_file = fs::read_to_string(path_str);
                    let objects: RoomObjectsFromFile =
                        serde_json::from_str(&objects_file.unwrap().as_str())
                            .expect("Error couldn't load objects");
                    let area_name = key.split('_').collect::<Vec<&str>>()[0];

                    let mut objects_map: HashMap<String, Vec<Arc<dyn Obj>>> = HashMap::new();

                    objects.insert_all(area_name, &mut objects_map);

                    let room_objects = RoomObjects {
                        objects: objects_map,
                    };

                    res.insert_object(area_name.to_string(), Arc::new(room_objects));
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

pub struct RoomObjects {
    pub objects: HashMap<String, Vec<Arc<dyn Obj>>>,
}

use crate::cutscene::Cutscene;
use crate::level::Room;
use crate::room_obj_from_file::RoomObjectsFromFile;
use crate::traits_for_obj::Obj;
use macroquad::prelude::Texture2D;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use walkdir::WalkDir;

const ERROR_TEXT: &str = "ERROR, STRING NOT FOUND";

pub struct Resources {
    pub rooms: HashMap<String, Arc<Room>>,
    pub room_objects: HashMap<String, Arc<RoomObjects>>,
    pub textures: HashMap<String, Texture2D>,
    pub text_strings: HashMap<String, String>,
    pub cutscenes: HashMap<String, Arc<Cutscene>>,
    pub background_texture: Option<String>,
}

pub static RESOURCE_MANAGER: OnceCell<Resources> = OnceCell::new();

impl Resources {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            room_objects: HashMap::new(),
            textures: HashMap::new(),
            background_texture: None,
            text_strings: HashMap::new(),
            cutscenes: HashMap::new(),
        }
    }

    pub fn global() -> &'static Resources {
        RESOURCE_MANAGER.get().expect("resources not initialised")
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
        self.rooms.get(key).cloned()
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

    pub fn insert_text(&mut self, key: String, text: String) {
        self.text_strings.insert(key, text);
    }

    fn get_text(&self, key: &str) -> &str {
        if let Some(text) = self.text_strings.get(key) {
            text
        } else {
            ERROR_TEXT
        }
    }

    pub fn insert_cutscene(&mut self, key: String, cutscene: Arc<Cutscene>) {
        self.cutscenes.insert(key, cutscene);
    }

    pub fn get_cutscene(&self, key: &str) -> Option<Arc<Cutscene>> {
        self.cutscenes.get(key).cloned()
    }
}

pub async fn load_all_assets() {
    let mut res = Resources::new();

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
                    //tile map json
                    let level = Room::build(path_str).await.unwrap();
                    res.insert_room(key.to_string(), Arc::new(level));
                    println!("{}", key);
                }
                "roomobj" => {
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
                "gametext" => {
                    //game text json
                    let text_file = fs::read_to_string(path_str).unwrap();
                    let map: HashMap<String, String> = serde_json::from_str(&text_file).unwrap();
                    for (key, value) in map {
                        res.insert_text(key, value);
                    }
                    println!("{:?}", res.text_strings);
                }
                "cutscene" => {
                    let file = fs::read_to_string(path_str).unwrap();
                    let cutscene: Cutscene = serde_json::from_str(&file).unwrap();
                    res.insert_cutscene(key.to_string(), cutscene.into());
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
    let _ = RESOURCE_MANAGER.set(res);
}

pub fn get_text(key: &str) -> &str {
    let text = {
        let res = Resources::global();
        res.get_text(key)
    };
    text
}

pub struct RoomObjects {
    pub objects: HashMap<String, Vec<Arc<dyn Obj>>>,
}

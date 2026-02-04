use crate::level::Room;
use crate::obj__trait::Obj;
use crate::obj_checkpoint::Checkpoint;
use crate::obj_door::{DoorDestination, DoorInGame};
use crate::obj_npc::NpcInGame;
use macroquad::math::{IVec2, ivec2, vec2};
use macroquad::prelude::Texture2D;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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

                    fn insert_from_list<T: FileToInGame>(
                        list: Vec<T>,
                        area_name: &str,
                        map: &mut HashMap<String, Vec<Arc<dyn Obj>>>,
                    ) {
                        for item in list {
                            let coords = item.room_coords();
                            let room = format!("{}_{}", coords.x, coords.y);
                            map.entry(room)
                                .or_insert_with(Vec::new)
                                .push(item.to_obj(area_name));
                        }
                    }
                    insert_from_list(objects.checkpoints, area_name, &mut objects_map);
                    insert_from_list(objects.npcs, area_name, &mut objects_map);
                    insert_from_list(objects.doors, area_name, &mut objects_map);

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomObjectsFromFile {
    pub checkpoints: Vec<CheckpointFromFile>,
    pub npcs: Vec<NpcFromFile>,
    pub doors: Vec<DoorFromFile>,
}

pub trait FileToInGame {
    fn room_coords(&self) -> IVec2;
    fn to_obj(&self, area_name: &str) -> Arc<dyn Obj>;
}

#[derive(Serialize, Deserialize, Debug)]
struct DoorLocation {
    room_x: i32,
    room_y: i32,
    pos_x: i32,
    pos_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoorFromFile {
    id: i32,
    location: DoorLocation,
    destination: DoorDestination,
    visible: bool,
    need_interact: bool,
}

impl FileToInGame for DoorFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.location.room_x, self.location.room_y)
    }

    fn to_obj(&self, _area_name: &str) -> Arc<dyn Obj> {
        Arc::new(DoorInGame::new(
            vec2(
                self.location.pos_x as f32 * 32.,
                self.location.pos_y as f32 * 32.,
            ),
            self.destination.clone(),
            self.visible,
            self.need_interact,
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NpcFromFile {
    id: i32,
    name: String,
    room_x: i32,
    room_y: i32,
    pos_x: i32,
    pos_y: i32,
}

impl FileToInGame for NpcFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, _area_name: &str) -> Arc<dyn Obj> {
        Arc::new(NpcInGame::new(
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
            self.name.clone(),
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckpointFromFile {
    pub id: i32,
    pub room_x: i32,
    pub room_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl FileToInGame for CheckpointFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, area_name: &str) -> Arc<dyn Obj> {
        Arc::new(Checkpoint::new(
            self.id,
            area_name.to_string(),
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
        ))
    }
}

use std::fs;
use macroquad::prelude::{load_texture, Texture2D};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TiledLayer {
    data: Vec<i32>,
    height: i32,
    id: i32,
    name: String,
    opacity: i32,
    r#type: String,
    visible: bool,
    width: i32,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct TiledMapSet {
    firstgid: i32,
    source: String,
}

#[derive(Serialize, Deserialize)]
struct TiledMap {
    compressionlevel: i32,
    height: i32,
    infinite: bool,
    layers: Vec<TiledLayer>,
    nextlayerid: i32,
    nextobjectid: i32,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i32,
    tilesets: Vec<TiledMapSet>,
    tilewidth: i32,
    r#type: String,
    version: String,
    width: i32,
}

#[derive(Serialize, Deserialize)]
struct TiledTileSet {
    columns: i32,
    image: String,
    imageheight: i32,
    imagewidth: i32,
    margin: i32,
    name: String,
    spacing: i32,
    tilecount: i32,
    tiledversion: String,
    tileheight: i32,
    tilewidth: i32,
    r#type: String,
    version: String,
}

//game level
pub(crate) struct Level {
    pub(crate) tile_image: Texture2D,
    pub(crate) tile_values: Vec<i32>,
}

impl Level {
    pub(crate) async fn build(map_name: &str) -> Result<Level, &'static str > {
        //load tiles
        let tile_map_file = fs::read_to_string(map_name).unwrap();
        let tile_map: TiledMap = serde_json::from_str(&tile_map_file).unwrap();

        let tileset_file_name: Vec<&str> = tile_map
            .tilesets
            .first()
            .unwrap()
            .source
            .split('.')
            .collect();
        let tileset_file = fs::read_to_string(format!("{}.json", tileset_file_name.first().unwrap()));
        let tileset: TiledTileSet = serde_json::from_str(&tileset_file.unwrap()).unwrap();

        let values = tile_map.layers.first().unwrap().data.clone();

        let tile_image = load_texture(&tileset.image).await.unwrap();
        Ok(Level {
            tile_image,
            tile_values: values,
        })
    }
}
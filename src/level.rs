use std::fs;
use std::path::Path;
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
struct TiledSetTiles {
    id: i32,
    r#type: String,
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
    tiles: Vec<TiledSetTiles>,
    tilewidth: i32,
    r#type: String,
    version: String,
}

//game level
pub struct Level {
    pub tile_image_name: String,
    pub tile_values: Vec<i32>,
    pub map_dimensions: (f32, f32),
    pub tile_size: f32,
    pub tileset_columns: i32,
}

impl Level {
    pub async fn build(map_name: &str) -> Result<Level, Box<dyn std::error::Error> > {
        let dir = "assets/maps/";
        //load tiles
        let tile_map_file = fs::read_to_string(map_name)?;
        let tile_map: TiledMap = serde_json::from_str(&tile_map_file)?;

        let dimensions = (tile_map.width as f32, tile_map.height as f32);
        let tile_size = tile_map.tilewidth as f32;

        let tileset_file_name = dir.to_string() + &tile_map
            .tilesets
            .first()
            .ok_or("No tileset")?
            .source
            .clone();

        let tileset_path = Path::new(&tileset_file_name);
        let tileset_file = fs::read_to_string(tileset_path)?;
        let tileset: TiledTileSet = serde_json::from_str(&tileset_file)?;

        let values = tile_map.layers.first().ok_or("No layer")?.data.clone();

        let tile_image_name = tileset.image;

        let tileset_columns = tileset.columns;

        Ok(Level {
            tile_image_name,
            tile_values: values,
            map_dimensions: dimensions,
            tile_size,
            tileset_columns,
        })
    }
}
// for level info, such as tiles etc. other stuff handled in level state (level_state.rs)

use crate::coords_to_index;
use macroquad::math::{IVec2, ivec2};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// all these serialize/deserialize ones are for representing the tiled map json files
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
struct TileProperties {
    name: String,
    r#type: String,
    value: bool,
}

#[derive(Serialize, Deserialize)]
struct TiledSetTiles {
    id: i32,
    properties: Vec<TileProperties>,
}

#[derive(Serialize, Deserialize)]
pub struct TiledTileSet {
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
pub struct Room {
    pub tile_image_name: String,
    pub tile_values: Vec<i32>,
    pub map_dimensions: IVec2,
    pub tile_size: i32,
    pub tileset_columns: i32,
    pub tileset: TiledTileSet,
    pub x_coord: i32,
    pub y_coord: i32,
}

#[derive(Debug)]
pub struct TileInfo {
    pub solid: bool,
    pub ladder: bool,
    pub hazard: bool,
}

impl Room {
    pub async fn build(map_name: &str) -> Result<Room, Box<dyn std::error::Error>> {
        let dir = "assets/maps/";
        let map_name_parts: Vec<&str> = map_name.split("_").collect();
        let x_coord: i32 = map_name_parts[1].parse()?;
        let y_coord: i32 = map_name_parts[2].split(".").collect::<Vec<&str>>()[0].parse()?;
        //load tiles
        let tile_map_file = fs::read_to_string(map_name)?;
        let tile_map: TiledMap = serde_json::from_str(&tile_map_file)?;

        let dimensions = ivec2(tile_map.width, tile_map.height);
        let tile_size = tile_map.tilewidth;

        let tileset_file_name = dir.to_string()
            + &tile_map
                .tilesets
                .first()
                .ok_or("No tileset")?
                .source
                .clone();

        let tileset_path = Path::new(&tileset_file_name);
        let tileset_file = fs::read_to_string(tileset_path)?;
        let tileset: TiledTileSet = serde_json::from_str(&tileset_file)?;

        let values = tile_map.layers.first().ok_or("No layer")?.data.clone();

        let tile_image_name = tileset.image.clone();

        let tileset_columns = tileset.columns;

        Ok(Room {
            tile_image_name,
            tile_values: values,
            map_dimensions: dimensions,
            tile_size,
            tileset_columns,
            tileset,
            x_coord,
            y_coord,
        })
    }

    pub fn get_tile_info(&self, tile_coords: IVec2) -> TileInfo {
        if tile_coords.x < 0 || tile_coords.y < 0 {
            return TileInfo {
                solid: false,
                ladder: false,
                hazard: false,
            };
        }
        if tile_coords.x > self.map_dimensions.x - 1 || tile_coords.y > self.map_dimensions.y - 1 {
            return TileInfo {
                solid: false,
                ladder: false,
                hazard: false,
            };
        }
        let index = coords_to_index(tile_coords.x, tile_coords.y, self.map_dimensions.x);
        let tile = self.tile_values[index as usize];
        let tileset = &self.tileset;
        let tile_properties = &tileset.tiles[tile as usize - 1].properties;
        let mut solid: bool = false;
        let mut ladder: bool = false;
        let mut hazard: bool = false;
        for i in tile_properties {
            if i.name == "solid" && i.value {
                solid = true;
            }
            if i.name == "ladder" && i.value {
                ladder = true;
            }
            if i.name == "hazard" && i.value {
                hazard = true;
            }
        }
        TileInfo {
            solid,
            ladder,
            hazard,
        }
    }
}

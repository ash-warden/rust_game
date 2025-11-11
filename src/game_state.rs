use macroquad::color::WHITE;
use macroquad::math::{vec2, Rect};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams};
use crate::{index_to_coords, level};

pub trait GameState {
    fn update(&mut self);
    fn draw(&self, scale: f32);
}

pub struct LevelState {
    pub level: level::Level,
}

impl LevelState {
    pub async fn build() -> Result<LevelState, Box<dyn std::error::Error>> {
        let level = level::Level::build("assets/maps/test_map1.json")
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to load level: {err}");
                std::process::exit(1);
            });
        Ok(LevelState { level })
    }
}
impl GameState for LevelState {
    fn update(&mut self) {}
    fn draw(&self, scale: f32) {
        //draw tiles
        let mut i = 0; //tile number
        let mut x = 0.; //x coord
        let mut y = 0.; //y coord

        let level = &self.level;

        let map_width = level.map_dimensions.0;
        let map_height = level.map_dimensions.1;

        while y < map_height {
            //column
            while x < map_width {
                //row
                draw_texture_ex(
                    &level.tile_image,
                    x * level.tile_size * scale,
                    y * level.tile_size * scale,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(level.tile_size * scale, level.tile_size * scale)),
                        source: Some(Rect::new(
                            index_to_coords(level.tile_values[i], level.tileset_columns).0
                                * level.tile_size,
                            index_to_coords(level.tile_values[i], level.tileset_columns).1
                                * level.tile_size,
                            level.tile_size,
                            level.tile_size,
                        )),
                        ..Default::default()
                    },
                );
                x += 1.;
                if i >= level.tile_values.len() {
                    println!("too many tiles to draw!");
                    break;
                }
                i += 1;
            }
            x = 0.; // go back to beginning of row
            y += 1.;
        }
    }
}

pub struct GameStateStack {
    pub states: Vec<Box<dyn GameState>>,
}

impl GameStateStack {
    pub fn new(initial: Box<dyn GameState>) -> Self {
        Self { states: vec![initial] }
    }

    pub fn update(&mut self) {
        if let Some(state) = self.states.last_mut() {
            state.update();
        }
    }

    pub fn draw(&self, scale: f32) {
        if let Some(state) = self.states.last() {
            state.draw(scale);
        }
    }

    pub fn push(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn replace(&mut self, state: Box<dyn GameState>) {
        self.pop();
        self.push(state);
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}
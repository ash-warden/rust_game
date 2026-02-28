// name in json, name of struct
// @start_room_object_info
// npcs,NpcFromFile
// @end_room_object_info

use std::sync::Arc;

use crate::controls::CONTROLS;
use crate::game_state::{GameState, MenuState, StateTransition};
use crate::menu::{Menu, MenuItem, menu_centre_pos};
use crate::resources::{Resources, get_text};
use crate::text::write_text;
use crate::traits_for_obj::{FileToInGame, Obj};
use macroquad::color::{Color, WHITE};
use macroquad::math::{IVec2, Vec2, ivec2, vec2};
use macroquad::shapes::draw_rectangle;
use serde::{Deserialize, Serialize};

pub struct TalkState {
    pub lines: Vec<String>,
    pub function: Option<Box<dyn Fn() -> ()>>,
    pub current_line: usize,
}

impl TalkState {
    pub fn new(lines_name: &str, fun: Option<Box<dyn Fn() -> ()>>) -> Self {
        TalkState {
            lines: Resources::global().get_npc_dialog(lines_name).unwrap(),
            function: fun,
            current_line: 0,
        }
    }
}

impl GameState for TalkState {
    fn update(&mut self) -> StateTransition {
        let mut controls = CONTROLS.lock().unwrap();
        if controls.controls_tertirary_release() {
            if (self.current_line as usize) < (self.lines.len() - 1) {
                self.current_line += 1;
                if self.lines[self.current_line] == "fn" {
                    if let Some(fun) = &self.function {
                        fun();
                    }
                    self.current_line += 1;
                }
            } else {
                return StateTransition::Pop(1);
            }
        }
        StateTransition::None
    }
    fn draw(&self) {
        draw_rectangle(16., 304., 608., 160., Color::new(0.1, 0.1, 0.1, 1.));
        write_text(
            &self.lines[self.current_line],
            vec2(32., 320.),
            WHITE,
            "font.png",
        );
    }
    fn transparent(&self) -> bool {
        true
    }
}

fn npc_function(npc_name: &str) -> StateTransition {
    match npc_name {
        "Test1" => StateTransition::Push(Box::new(TalkState::new(
            "test1",
            Some(Box::new(|| println!("function!"))),
        ))),
        _ => {
            println!("Error, no npc found");
            StateTransition::None
        }
    }
}

#[derive(Clone, Debug)]
pub struct NpcInGame {
    pub pos: Vec2,
    pub size: Vec2,
    pub npc_type: String,
    pub image: String,
}

impl NpcInGame {
    pub fn new(pos: Vec2, npc_type: String, image: String, size: Vec2) -> Self {
        NpcInGame {
            pos,
            size,
            npc_type,
            image,
        }
    }
}

impl Obj for NpcInGame {
    fn interact(&self) -> StateTransition {
        npc_function(&self.npc_type)
    }

    fn contact(&self) -> StateTransition {
        StateTransition::None
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }
    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_hud_text(&self) -> &str {
        get_text("obj_npc_hud")
    }

    fn get_tex(&self) -> &str {
        &self.image
    }

    fn is_visible(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NpcFromFile {
    id: i32,
    name: String,
    image: String,
    room_x: i32,
    room_y: i32,
    pos_x: i32,
    pos_y: i32,
    size_x: f32,
    size_y: f32,
}

impl FileToInGame for NpcFromFile {
    fn room_coords(&self) -> IVec2 {
        ivec2(self.room_x, self.room_y)
    }

    fn to_obj(&self, _area_name: &str) -> Arc<dyn Obj> {
        Arc::new(NpcInGame::new(
            vec2(self.pos_x as f32 * 32., self.pos_y as f32 * 32.),
            self.name.clone(),
            self.image.clone(),
            vec2(self.size_x, self.size_y),
        ))
    }
}

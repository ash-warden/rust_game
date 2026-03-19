use std::sync::Arc;

use macroquad::{
    math::{IVec2, vec2},
    time::get_frame_time,
};

use crate::{
    controls::CONTROLS,
    game_state::{GameState, MenuState, StateTransition},
    level_state::LevelState,
    menu::{Menu, MenuItem, menu_centre_pos},
    player::PlayerInitialInfo,
    resources::{Resources, get_text},
    traits_for_obj::Obj,
};

pub struct AreaState {
    pub area_name: String,
    pub objects: Vec<Arc<dyn Obj>>,
    pub current_level_state: LevelState,
}

impl AreaState {
    pub fn build(
        level: &str,
        player_info: PlayerInitialInfo,
    ) -> Result<AreaState, Box<dyn std::error::Error>> {
        // basically, load all the area objects and store them
        // then load the first level state
        // level states then switch on their own
        // what about switching areas?
        let area = level.split_once('_').unwrap().0;

        let level_state = LevelState::build(level, player_info).unwrap();
        let objects = Resources::global().get_room_object(&area).unwrap();
        let mut objects_array: Vec<Arc<dyn Obj>> = vec![];
        for i in objects.objects.values() {
            for j in i {
                objects_array.push(j.clone());
            }
        }

        Ok(AreaState {
            area_name: area.to_string(),
            objects: objects_array,
            current_level_state: level_state,
        })
    }
}

#[derive(PartialEq)]
enum DirectionToMove {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl GameState for AreaState {
    fn update(&mut self) -> crate::game_state::StateTransition {
        //pausing
        {
            let pause_text = get_text("pause");
            let resume_text = get_text("resume");
            let quit_text = get_text("quit");

            let pause_menu_pos = menu_centre_pos(6, 3);
            let mut pause_menu = Menu::new(pause_menu_pos.x, pause_menu_pos.y);
            let title = MenuItem::new(&pause_text, || StateTransition::None, false);
            pause_menu.add_item(title);
            let resume_game = MenuItem::new(&resume_text, move || StateTransition::Pop(1), true);
            pause_menu.add_item(resume_game);
            let quit_game = MenuItem::new(&quit_text, move || StateTransition::Pop(2), true);
            pause_menu.add_item(quit_game);
            let menu_state = MenuState::new(pause_menu);
            let mut input = CONTROLS.lock().unwrap();
            if input.controls_enter_release() {
                return StateTransition::Push(Box::new(menu_state));
            }
        }

        let ls = &mut self.current_level_state;
        ls.update();

        //check for new level

        let map_width = (ls.room.map_dimensions.x * ls.room.tile_size) as f32;
        let map_height = (ls.room.map_dimensions.y * ls.room.tile_size) as f32;

        let p_size_x = ls.player.actual_size.x as f32;
        let p_size_y = ls.player.actual_size.y as f32;

        let direction = if ls.player.position.x < -p_size_x / 2. {
            DirectionToMove::Left
        } else if ls.player.position.x > map_width - p_size_x / 2. {
            DirectionToMove::Right
        } else if ls.player.position.y < -p_size_y / 2. {
            DirectionToMove::Up
        } else if ls.player.position.y > map_height - p_size_y / 2. {
            DirectionToMove::Down
        } else {
            DirectionToMove::None
        };

        if direction == DirectionToMove::None {
            let mut input = CONTROLS.lock().unwrap();
            let player = &ls.player;
            for obj in &ls.objects {
                let overlapping_x = player.position.x < obj.get_pos().x + obj.get_size().x
                    && player.position.x + player.actual_size.x as f32 > obj.get_pos().x;

                let overlapping_y = player.position.y < obj.get_pos().y + obj.get_size().y
                    && player.position.y + player.actual_size.y as f32 > obj.get_pos().y;

                if overlapping_x && overlapping_y {
                    let text = obj.get_hud_text();
                    ls.hud_hint_text = text.replace("KEY", input.key_string("z").as_str());
                    obj.contact();
                    if input.controls_tertirary_release() {
                        return obj.interact();
                    }
                }
            }

            let frame_time = get_frame_time();
            if !ls.init_timer_done {
                ls.hud_init_timer -= frame_time;
                if ls.full_hud && ls.hud_init_timer <= 0. {
                    ls.full_hud = false;
                    ls.init_timer_done = true;
                }
            }
            if input.controls_esc_release() {
                ls.full_hud = !ls.full_hud;
            }
            return StateTransition::None;
        }

        let (new_player_pos, offset) = match direction {
            DirectionToMove::Left => (
                vec2(map_width - p_size_x / 2. - 1., ls.player.position.y),
                IVec2::new(-1, 0),
            ),
            DirectionToMove::Right => (
                vec2(-p_size_x / 2. + 1., ls.player.position.y - 1.),
                IVec2::new(1, 0),
            ),
            DirectionToMove::Up => (
                vec2(ls.player.position.x, map_height - p_size_y / 2. - 1.),
                IVec2::new(0, 1),
            ),
            DirectionToMove::Down => (
                vec2(ls.player.position.x, -p_size_y / 2. + 1.),
                IVec2::new(0, -1),
            ),
            DirectionToMove::None => unreachable!(),
        };

        let new_room = format!(
            "{}_{}_{}",
            ls.area_name,
            ls.room.x_coord + offset.x,
            ls.room.y_coord + offset.y
        );

        let player_info = PlayerInitialInfo {
            pos: new_player_pos,
            velocity: ls.player.velocity,
            state: ls.player.state.clone(),
        };

        // Load new LevelState
        match LevelState::build(&new_room, player_info) {
            Ok(new_level_state) => self.current_level_state = new_level_state,
            Err(err) => {
                eprintln!("Failed to load level state \"{}\": {err}", &new_room);
                std::process::exit(1);
            }
        }

        StateTransition::None
    }

    fn draw(&self) {
        self.current_level_state.draw();
    }

    fn transparent(&self) -> bool {
        false
    }
}

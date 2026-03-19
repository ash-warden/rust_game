use std::sync::Arc;

use crate::{
    resources::{Resources, RoomObjects},
    traits_for_obj::Obj,
};

pub struct Area {
    pub area_name: String,
    pub objects: Arc<RoomObjects>,
}

impl Area {
    pub fn build(level: &str) -> Area {
        let area = level.split_once('_').unwrap().0;

        let objects = Resources::global().get_room_object(&area).unwrap();
        Area {
            area_name: area.to_string(),
            objects: objects,
        }
    }

    pub fn get_obj_for_room(&self, room_x: i32, room_y: i32) -> Option<&Vec<Arc<dyn Obj>>> {
        println!("{}_{}", room_x, room_y);
        self.objects.objects.get(&format!("{}_{}", room_x, room_y))
    }
}

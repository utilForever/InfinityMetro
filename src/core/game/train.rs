use crate::core::game::room::Room;

pub struct Train {
    id: u32,
    pub count: u8,
    pub type_: u8,
    pub room: Room,
    pub loc_func: fn(f32, u32) -> f32,
}

impl Train {
    fn new(id: u32, type_: u8, loc_func: fn(f32, u32) -> f32) -> Train {
        Train {
            id,
            count: 0,
            type_,
            room: Room::new(),
            loc_func,
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod train_test {}

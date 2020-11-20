use crate::core::game::room::Room;

pub struct Station {
    id: u32,
    pub type_: u8,
    pub max_people: u8,
    pub room: Room,
}

impl Station {
    pub fn new(id: u32, type_: u8, max_people: u8) -> Station {
        Station {
            id,
            type_,
            max_people,
            room: Room::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod station_test {
    use super::*;
}

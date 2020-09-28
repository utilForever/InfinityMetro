use crate::core::game::room::Room;

pub struct Train {
    id: u32,
    count: u8,
    type_: u8,
    room: Room,
}

use crate::core::game::station::Station;
use crate::core::game::train::Train;

pub struct Route<'a> {
    stations: Vec<&'a Station>,
    trains: Vec<Train>,
}

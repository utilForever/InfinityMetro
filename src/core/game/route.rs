use crate::core::game::station::Station;
use crate::core::game::train::Train;

pub struct Route<'a> {
    stations: Vec<&'a Station>,
    trains: Vec<Train>,
}

impl<'a> Route<'a> {
    fn new() -> Route<'a> {
        Route {
            stations: vec![],
            trains: vec![],
        }
    }
}

#[cfg(test)]
mod route_test {
    use super::*;
}

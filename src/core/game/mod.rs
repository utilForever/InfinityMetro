use crate::core::game::route::Route;
use crate::core::game::station::Station;

mod room;
mod route;
mod station;
mod train;

pub struct Game<'a> {
    time: u64,
    score: u32,
    map: u8,
    stations: Vec<Station>,
    routes: Vec<Route<'a>>,
}

#[cfg(test)]
mod game_test {}

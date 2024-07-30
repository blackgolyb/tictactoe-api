// #[macro_use]
// extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::types::{GameMap, Room};

use super::traits::GameRepositoryInterface;

// lazy_static! {
//     static ref GAME_DB: Mutex<HashMap<Room, &'static GameMap>> = Mutex::new(HashMap::new());
// }

// static games: HashMap<Room, GameMap> = map.get_or_init(Default::default);

pub struct InMemoryGameRepository {}

impl GameRepositoryInterface for InMemoryGameRepository {
    fn get_board(&self, room: Room) -> Option<GameMap> {
        // let db = GAME_DB.lock().unwrap();
        // db.get(&room).copied()
        unimplemented!()
    }
    fn update_board(&mut self, room: Room, board: GameMap) {
        // let db = GAME_DB.lock().unwrap();
        // db.insert(room, &board);
        unimplemented!()
    }
}

impl InMemoryGameRepository {
    pub fn new() -> Self {
        Self {}
    }
}

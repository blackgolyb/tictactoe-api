use crate::core::types::{GameMap, Room};

pub trait GameRepositoryInterface {
    fn get_board(&self, room: Room) -> Option<GameMap>;
    fn update_board(&mut self, room: Room, board: GameMap);
}
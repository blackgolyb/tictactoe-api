use crate::core::types::{Game, GameResult, Room};

pub trait GameRepositoryInterface {
    fn get_game(&self, room: Room) -> Option<Game>;
    fn update_game(&self, room: Room, game: Game) -> GameResult<()>;
}

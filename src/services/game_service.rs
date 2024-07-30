use crate::core::types::{FieldId, FieldStatus, GameStatus, Room, WinnerSequence};
use crate::repositories::traits::GameRepositoryInterface;

use super::traits::GameServiceInterface;

pub struct GameService {
    room_repository: Box<dyn GameRepositoryInterface>,
}

impl GameService {
    fn new(room_repository: Box<dyn GameRepositoryInterface>) -> Self {
        Self { room_repository }
    }
}

impl GameServiceInterface for GameService {
    fn get_field(&self, room: Room, field_id: FieldId) -> FieldStatus {
        unimplemented!()
    }
    fn make_step(&self, room: Room, field_id: FieldId) -> Result<(), String> {
        unimplemented!()
    }
    fn check_game(&self, room: Room) -> (GameStatus, Option<WinnerSequence>) {
        unimplemented!()
    }
}

use crate::core::types::{FieldId, FieldStatus, Game, GameStatus, Room, WinnerSequence};

pub trait GameServiceInterface {
    fn make_step(&self, room: Room, field_id: FieldId) -> Result<(), String>;
    fn check_game(&self, room: Room) -> (Game, GameStatus, Option<WinnerSequence>);
}

pub trait VisualizeGame {
    fn get_field_image(
        &self,
        field_id: FieldId,
        field_status: FieldStatus,
        winners_field: Option<WinnerSequence>,
    ) -> Vec<u8>;

    fn get_game_over_image(&self) -> Vec<u8>;
}

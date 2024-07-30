use crate::core::types::{FieldId, FieldStatus, GameStatus, Room, WinnerSequence};

pub trait GameServiceInterface {
    fn get_field(&self, room: Room, field_id: FieldId) -> FieldStatus;
    fn make_step(&self, room: Room, field_id: FieldId) -> Result<(), String>;
    fn check_game(&self, room: Room) -> (GameStatus, Option<WinnerSequence>);
}

pub trait VisualizeGame {
    fn get_field_image(
        &self,
        field_id: FieldId,
        field_status: FieldStatus,
        winners_field: Option<WinnerSequence>,
    ) -> Vec<u8>;
}

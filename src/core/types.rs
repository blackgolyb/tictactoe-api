pub type GameMap = Vec<u8>;
pub type FieldId = u8;
pub type Room = String;
pub type WinnerSequence = Vec<FieldId>;

pub enum FieldStatus {
    Empty,
    X,
    O,
}

pub enum GameStatus {
    Going,
    Draw,
    WinnerX,
    WinnerO,
}

pub enum LineType {
    Vertical,
    Horizontal,
    Diagonal1,
    Diagonal2,
}

use std::sync::{Arc, Mutex};

use actix_web::web;
use rusqlite::Connection;

pub type GameMap = Vec<FieldStatus>;
pub type FieldId = u8;
pub type Room = String;
pub type WinnerSequence = Vec<FieldId>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FieldStatus {
    Empty,
    O,
    X,
}

impl From<u8> for FieldStatus {
    fn from(status: u8) -> Self {
        match status {
            0 => FieldStatus::Empty,
            1 => FieldStatus::O,
            2 => FieldStatus::X,
            _ => unreachable!(),
        }
    }
}

impl From<FieldStatus> for u8 {
    fn from(status: FieldStatus) -> Self {
        match status {
            FieldStatus::Empty => 0,
            FieldStatus::O => 1,
            FieldStatus::X => 2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Draw,
    WinnerX,
    WinnerO,
}

#[derive(Copy, Clone, Debug)]
pub enum LineType {
    Vertical,
    Horizontal,
    Diagonal1,
    Diagonal2,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub board: GameMap,
    pub current_player: FieldStatus,
}

type Error = String;
pub type GameResult<T> = Result<T, Error>;

pub struct AppDependency {
    pub conn: Arc<Mutex<Connection>>,
}

pub type AppState = web::Data<AppDependency>;

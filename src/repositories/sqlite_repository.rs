use std::sync::MutexGuard;

use rusqlite::params;

use crate::{
    core::types::{FieldStatus, Game, GameMap, GameResult, Room},
    db::DBConnection,
};

use super::traits::GameRepositoryInterface;

pub struct SqliteGameRepository {
    conn: DBConnection,
}

impl SqliteGameRepository {
    pub fn new(conn: DBConnection) -> Self {
        // let c = DBConnection.lock() =
        Self { conn }
    }

    fn encode_game_map(&self, game_map: GameMap) -> u32 {
        let encode = |i: usize, a: u8| 3_u32.pow(i as u32) * a as u32;
        game_map
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, a)| encode(i, a.into()) + acc)
    }

    fn decode_game_map(&self, mut encoded: u32) -> GameMap {
        let mut res = vec![FieldStatus::Empty; 9];

        let mut i = 0;
        while encoded > 0 {
            let m = encoded % 3;
            encoded /= 3;
            res[i] = FieldStatus::from(m as u8);
            i += 1;
        }

        res
    }

    fn get_conn(&self) -> MutexGuard<rusqlite::Connection> {
        self.conn.lock().expect("Cannot get connection to database")
    }
}

impl GameRepositoryInterface for SqliteGameRepository {
    fn get_game(&self, room: Room) -> Option<Game> {
        let conn = self.get_conn();
        let mut stmt = conn
            .prepare("SELECT board, current_player FROM rooms WHERE name = ?1;")
            .unwrap();

        let result = stmt.query_row([room], |row| {
            let board: u32 = row.get(0)?;
            let current_player: u8 = row.get(1)?;
            Ok((board, current_player))
        });

        match result {
            Ok((board, current_player)) => Some(Game {
                board: self.decode_game_map(board),
                current_player: FieldStatus::from(current_player),
            }),
            Err(_) => None,
        }
    }
    fn update_game(&self, room: Room, game: Game) -> GameResult<()> {
        let conn = self.get_conn();
        let board = self.encode_game_map(game.board);

        let result = conn.execute(
            "INSERT OR REPLACE INTO rooms (name, board, current_player) VALUES (?1, ?2, ?3)",
            params![room, board, Into::<u8>::into(game.current_player)],
        );

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("lol".to_string()),
        }
    }
}

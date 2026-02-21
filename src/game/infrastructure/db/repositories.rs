use rusqlite::params;

use crate::game::{
    domain::{
        aggregates::Game,
        models::{FieldState, GameMap, GameRules, Player},
    },
    infrastructure::db::connection::DBConnection,
};

use crate::game::application::interfaces::GameRepository;

impl Player {
    pub fn encode(&self) -> u32 {
        match self {
            Player::O => 0,
            Player::X => 1,
        }
    }

    pub fn decode(raw: u32) -> Player {
        match raw {
            0 => Player::O,
            1 => Player::X,
            _ => unreachable!(),
        }
    }
}

impl FieldState {
    pub fn encode(&self) -> char {
        match self {
            FieldState::Occupied(Player::O) => 'O',
            FieldState::Occupied(Player::X) => 'X',
            FieldState::Empty => 'E',
        }
    }

    pub fn decode(c: char) -> FieldState {
        match c {
            'O' => FieldState::Occupied(Player::O),
            'X' => FieldState::Occupied(Player::X),
            _ => FieldState::Empty,
        }
    }
}

impl GameRules {
    pub fn decode(raw: String) -> GameRules {
        let mut parts = raw.split(';');
        let size_part = parts.next().unwrap_or("3,3");
        let win_len_part = parts.next().unwrap_or("3");

        let mut size_iter = size_part.split(',');
        let width = size_iter
            .next()
            .unwrap_or("3")
            .parse::<usize>()
            .unwrap_or(3);
        let height = size_iter
            .next()
            .unwrap_or("3")
            .parse::<usize>()
            .unwrap_or(3);
        let winning_length = win_len_part.parse::<u32>().unwrap_or(3);

        GameRules {
            game_size: (width, height),
            winning_length,
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "{},{};{}",
            self.game_size.0, self.game_size.1, self.winning_length
        )
    }
}

impl GameMap {
    pub fn decode(raw: String) -> GameMap {
        let data = raw
            .split(';')
            .map(|row| {
                row.chars()
                    .map(|c| FieldState::decode(c))
                    .collect::<Vec<FieldState>>()
            })
            .collect::<Vec<Vec<FieldState>>>();
        Self { data }
    }

    pub fn encode(&self) -> String {
        self.data
            .iter()
            .map(|row| row.iter().map(|field| field.encode()).collect::<String>())
            .collect::<Vec<String>>()
            .join(";")
    }
}

pub struct SqliteGameRepository {
    pool: DBConnection,
}

impl SqliteGameRepository {
    pub fn new(pool: DBConnection) -> Self {
        Self { pool }
    }
}

impl GameRepository for SqliteGameRepository {
    fn get_by_name(&self, name: &str) -> Option<Game> {
        let conn = self.pool.get().expect("Failed to get connection from pool");
        let mut stmt = conn
            .prepare("SELECT name, board, current_player, rules FROM Game WHERE name = ?1;")
            .unwrap();

        stmt.query_row([name], |row| {
            let name: String = row.get(0)?;
            let board: GameMap = row.get(1).map(GameMap::decode)?;
            let current_player: Player = row.get(2).map(Player::decode)?;
            let rules: GameRules = row.get(3).map(GameRules::decode)?;
            Ok(Game::load(name, board, current_player, rules))
        })
        .ok()
    }

    fn save(&self, game: &Game) {
        let conn = self.pool.get().expect("Failed to get connection from pool");

        let name = game.name();
        let board = game.board().encode();
        let current_player = game.current_player().encode();
        let rules = game.rules().encode();

        println!(
            "Saving game with params: name={}, board={}, current_player={}, rules={}",
            name, board, current_player, rules
        );

        conn.execute(
            "
                INSERT INTO Game (name, board, current_player, rules)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT (name) DO UPDATE SET
                    board = EXCLUDED.board,
                    current_player = EXCLUDED.current_player,
                    rules = EXCLUDED.rules;
                ",
            params![name, board, current_player, rules],
        )
        .expect("Game repo fails");
    }
}

use crate::shared::utils::iterator::find_consecutive_sequence;
use crate::shared::utils::matrix::{
    iterate_all_cells, iterate_diagonal_ascending, iterate_diagonal_descending, iterate_horizontal,
    iterate_vertical,
};

use super::models::{FieldState, GameMap, GameRules, GameState, Player};
use super::value_objects::Segment;

#[derive(Clone, Debug, PartialEq)]
pub enum GameError {
    FieldAlreadyOccupaed,
    StepIsOutOfTheBoard,
    GameAlreadyEnded,
}

// enum GameEvents {
//     Ended
// }

#[derive(Clone, Debug)]
pub struct Game {
    name: String,
    board: GameMap,
    current_player: Player,
    rules: GameRules,
    // events: Vec<GameEvents>
}

impl Game {
    pub fn new(name: String, rules: GameRules, first_player: Player) -> Self {
        let (w, h) = rules.game_size;
        Self {
            name,
            rules,
            current_player: first_player,
            board: GameMap::new(w, h),
        }
    }

    pub fn load(name: String, board: GameMap, current_player: Player, rules: GameRules) -> Self {
        Self {
            name,
            board,
            current_player,
            rules,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn board(&self) -> &GameMap {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn rules(&self) -> &GameRules {
        &self.rules
    }

    pub fn restart(&mut self) {
        self.board.clear();
        self.current_player = Player::O;
    }

    pub fn make_move(&mut self, x: usize, y: usize) -> Result<(), GameError> {
        if !self.check_move_fit_the_board(x, y) {
            return Err(GameError::StepIsOutOfTheBoard);
        }
        if self.check_game_status() != GameState::InProgress {
            self.restart();
        }
        if self.make_move_as_current_player(x, y).is_err() {
            return Err(GameError::FieldAlreadyOccupaed);
        };
        // self.events.push()
        self.change_current_player();
        Ok(())
    }

    pub fn check_game_status(&self) -> GameState {
        if self.check_draw() {
            return GameState::Draw;
        }
        self.check_vertical()
            .or_else(|| self.check_horizontal())
            .or_else(|| self.check_diagonal_ascending())
            .or_else(|| self.check_diagonal_descending())
            .unwrap_or(GameState::InProgress)
    }

    fn check_move_fit_the_board(&self, x: usize, y: usize) -> bool {
        let (w, h) = self.board.size();
        println!("{}, {}", w, h);
        w > x && h > y
    }

    fn change_current_player(&mut self) {
        self.current_player = match self.current_player {
            Player::O => Player::X,
            Player::X => Player::O,
        };
    }

    fn check_if_field_is_available(&self, x: usize, y: usize) -> bool {
        self.board.get(x, y) == FieldState::Empty
    }

    fn make_move_as_current_player(&mut self, x: usize, y: usize) -> Result<(), ()> {
        self.board.set(x, y, self.current_player)
    }

    fn check_draw(&self) -> bool {
        let (w, h) = self.board.size();
        iterate_all_cells(w, h)
            .map(|(x, y)| self.board.get(x, y))
            .all(|f| f != FieldState::Empty)
    }

    fn check_with<I, J>(&self, line_iter: I) -> Option<GameState>
    where
        I: Iterator<Item = J>,
        J: Iterator<Item = (usize, usize)>,
    {
        let check_line = |line| {
            find_consecutive_sequence(line, self.rules.winning_length, |(_, a), (_, b)| {
                a == b && a != &FieldState::Empty
            })
        };
        let extract_player = |f| match f {
            FieldState::Occupied(p) => Some(p),
            _ => None,
        };

        line_iter
            .map(|line| line.map(|(x, y)| ((x, y), self.board.get(x, y))))
            .map(check_line)
            .flatten()
            .next()
            .and_then(|((start, state), (end, _))| {
                Some(GameState::Winner(
                    extract_player(state)?,
                    Segment::new(
                        (start.0 as u64, start.1 as u64),
                        (end.0 as u64, end.1 as u64),
                    )
                    .expect("Unrepresentable winner sequence"),
                ))
            })
    }

    fn check_horizontal(&self) -> Option<GameState> {
        let (w, h) = self.board.size();
        self.check_with(iterate_horizontal(w, h))
    }

    fn check_vertical(&self) -> Option<GameState> {
        let (w, h) = self.board.size();
        self.check_with(iterate_vertical(w, h))
    }

    fn check_diagonal_ascending(&self) -> Option<GameState> {
        let (w, h) = self.board.size();
        self.check_with(iterate_diagonal_ascending(w, h))
    }

    fn check_diagonal_descending(&self) -> Option<GameState> {
        let (w, h) = self.board.size();
        self.check_with(iterate_diagonal_descending(w, h))
    }
}

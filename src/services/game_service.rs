use crate::core::types::{FieldId, FieldStatus, Game, GameMap, GameStatus, Room, WinnerSequence};
use crate::repositories::traits::GameRepositoryInterface;

use super::traits::GameServiceInterface;

pub struct GameService {
    repo: Box<dyn GameRepositoryInterface>,
}

impl GameService {
    pub fn new(repo: Box<dyn GameRepositoryInterface>) -> Self {
        Self { repo }
    }

    fn is_field_empty(board: &GameMap, i: usize) -> bool {
        board[i] == FieldStatus::Empty
    }

    fn check_horizontal(board: &GameMap, i: usize) -> bool {
        let i = i * 3;
        let three_in_row = board[i] == board[i + 1] && board[i + 1] == board[i + 2];
        three_in_row & !Self::is_field_empty(board, i)
    }

    fn get_win_horizontal(i: usize) -> WinnerSequence {
        let i = i * 3;
        vec![i as u8, (i + 1) as u8, (i + 2) as u8]
    }

    fn check_vertical(board: &GameMap, i: usize) -> bool {
        let three_in_col = board[i] == board[i + 3] && board[i + 3] == board[i + 6];
        three_in_col & !Self::is_field_empty(board, i)
    }

    fn get_win_vertical(i: usize) -> WinnerSequence {
        vec![i as u8, (i + 3) as u8, (i + 6) as u8]
    }

    fn get_winner(board: &GameMap, i: usize) -> GameStatus {
        match board[i] {
            FieldStatus::O => GameStatus::WinnerO,
            FieldStatus::X => GameStatus::WinnerX,
            _ => GameStatus::NotStarted,
        }
    }

    fn check_winners_and_status(&self, game: &Game) -> (GameStatus, Option<WinnerSequence>) {
        let board = &game.board;

        for i in 0..3 {
            if Self::check_horizontal(&board, i) {
                return (
                    Self::get_winner(board, i * 3),
                    Some(Self::get_win_horizontal(i * 3)),
                );
            }
            if Self::check_vertical(&board, i) {
                return (Self::get_winner(board, i), Some(Self::get_win_vertical(i)));
            }
        }

        let is_middle_empty = Self::is_field_empty(&board, 4);

        if board[0] == board[4] && board[4] == board[8] && !is_middle_empty {
            return (Self::get_winner(board, 4), Some(vec![0, 4, 8]));
        }

        if board[2] == board[4] && board[4] == board[6] && !is_middle_empty {
            return (Self::get_winner(board, 4), Some(vec![2, 4, 6]));
        }

        if board.contains(&FieldStatus::Empty) {
            (GameStatus::InProgress, None)
        } else {
            (GameStatus::Draw, None)
        }
    }

    fn check_is_occupied(&self, field_id: FieldId, board: &GameMap) -> bool {
        board[field_id as usize] != FieldStatus::Empty
    }
}

fn create_default_game() -> Game {
    Game {
        board: vec![FieldStatus::Empty; 9],
        current_player: FieldStatus::O,
    }
}

impl GameServiceInterface for GameService {
    fn make_step(&self, room: Room, field_id: FieldId) -> Result<(), String> {
        let (mut game, status, _) = self.check_game(room.clone());

        game = match status {
            GameStatus::InProgress => game,
            _ => create_default_game(),
        };

        if self.check_is_occupied(field_id, &game.board) {
            return Err("Field is already occupied".to_string());
        } else {
            game.board[field_id as usize] = game.current_player;
        }

        game.current_player = match game.current_player {
            FieldStatus::O => FieldStatus::X,
            FieldStatus::X => FieldStatus::O,
            _ => unreachable!(),
        };

        self.repo.update_game(room, game)?;
        Ok(())
    }

    fn check_game(&self, room: Room) -> (Game, GameStatus, Option<WinnerSequence>) {
        let game = self.repo.get_game(room);

        match game {
            None => (create_default_game(), GameStatus::NotStarted, None),
            Some(game) => {
                let (status, seq) = self.check_winners_and_status(&game);
                (game, status, seq)
            }
        }
    }
}

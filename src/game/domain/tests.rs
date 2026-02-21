use crate::game::domain::aggregates::{Game, GameError};
use crate::game::domain::models::{FieldState, GameMap, GameRules, Player};
use crate::game::domain::value_objects::SegmentDirection;

const X: Player = Player::X;
const O: Player = Player::O;
const FE: FieldState = FieldState::Empty;
const FX: FieldState = FieldState::Occupied(X);
const FO: FieldState = FieldState::Occupied(O);

impl GameMap {
    fn load(data: Vec<Vec<FieldState>>) -> Self {
        Self { data }
    }
}

fn default_game_rules() -> GameRules {
    GameRules {
        game_size: (3, 3),
        winning_length: 3,
    }
}

#[test]
fn test_game_initialization() {
    let game = Game::new("test_game".to_string(), default_game_rules(), X);
    assert_eq!(game.current_player(), X);
    let (w, h) = game.board().size();
    assert_eq!((w, h), (3, 3));
}

#[test]
fn test_make_move_and_switch_player() {
    let mut game = Game::new("test_game".to_string(), default_game_rules(), X);
    assert_eq!(game.make_move(0, 0), Ok(()));
    assert_eq!(game.board().get(0, 0), FX);
    assert_eq!(game.current_player(), O);
}

#[test]
fn test_make_move_out_of_bounds() {
    let mut game = Game::new("test_game".to_string(), default_game_rules(), X);
    assert_eq!(game.make_move(3, 3), Err(GameError::StepIsOutOfTheBoard));
}

#[test]
fn test_make_move_on_occupied_field() {
    let mut game = Game::new("test_game".to_string(), default_game_rules(), X);
    assert_eq!(game.make_move(0, 0), Ok(()));
    assert_eq!(game.make_move(0, 0), Err(GameError::FieldAlreadyOccupaed));
}

#[test]
fn test_horizontal_win() {
    let board = GameMap::load(vec![vec![FX, FX, FX], vec![FE, FO, FE], vec![FE, FE, FO]]);
    let game = Game::load("test_game".to_string(), board, X, default_game_rules());
    let state = game.check_game_status();
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, X);
            assert_eq!(segment.direction(), SegmentDirection::Horizontal);
            assert_eq!(segment.start(), (0, 0));
            assert_eq!(segment.end(), (2, 0));
        }
        _ => panic!("Expected horizontal win for X"),
    }
}

#[test]
fn test_vertical_win() {
    let board = GameMap::load(vec![vec![FO, FX, FE], vec![FO, FX, FE], vec![FO, FE, FX]]);
    let game = Game::load("test_game".to_string(), board, O, default_game_rules());
    let state = game.check_game_status();
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, O);
            assert_eq!(segment.direction(), SegmentDirection::Vertical);
            assert_eq!(segment.start(), (0, 0));
            assert_eq!(segment.end(), (0, 2));
        }
        _ => panic!("Expected vertical win for O"),
    }
}

#[test]
fn test_diagonal_descending_win() {
    let board = GameMap::load(vec![vec![FX, FO, FE], vec![FE, FX, FO], vec![FE, FE, FX]]);
    let game = Game::load("test_game".to_string(), board, X, default_game_rules());
    let state = game.check_game_status();
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, X);
            assert_eq!(segment.direction(), SegmentDirection::DiagonalDescending);
            assert_eq!(segment.start(), (0, 0));
            assert_eq!(segment.end(), (2, 2));
        }
        _ => panic!("Expected descending diagonal win for X"),
    }
}

#[test]
fn test_diagonal_ascending_win() {
    let board = GameMap::load(vec![vec![FE, FE, FO], vec![FE, FO, FX], vec![FO, FX, FX]]);
    let game = Game::load("test_game".to_string(), board, O, default_game_rules());
    let state = game.check_game_status();
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, O);
            assert_eq!(segment.direction(), SegmentDirection::DiagonalAscending);
            assert_eq!(segment.start(), (0, 2));
            assert_eq!(segment.end(), (2, 0));
        }
        _ => panic!("Expected ascending diagonal win for O"),
    }
}

#[test]
fn test_game_with_pre_existing_board() {
    let board = GameMap::load(vec![vec![FX, FE, FE], vec![FE, FO, FE], vec![FE, FE, FX]]);

    let mut game = Game::load("test_game".to_string(), board, O, default_game_rules());

    // Try to make a move on an occupied cell
    assert_eq!(game.make_move(0, 0), Err(GameError::FieldAlreadyOccupaed));

    // Make a valid move
    assert_eq!(game.make_move(0, 1), Ok(()));
    assert_eq!(game.board().get(0, 1), FO);
}

#[test]
fn test_game_map_positioning() {
    let board = GameMap::load(vec![vec![FO, FE, FE], vec![FE, FE, FX], vec![FE, FE, FE]]);
    assert_eq!(board.get(0, 0), FO);
    assert_eq!(board.get(2, 1), FX);
}

#[test]
fn test_empty_game_should_be_in_progress() {
    let game = Game::new("test_game".to_string(), default_game_rules(), X);
    let state = game.check_game_status();
    assert_eq!(state, crate::game::domain::models::GameState::InProgress);
}

#[test]
fn test_draw_game() {
    // Board with no winner but all cells occupied
    let board = GameMap::load(vec![vec![FX, FO, FX], vec![FO, FO, FX], vec![FO, FX, FO]]);
    let game = Game::load("test_game".to_string(), board, X, default_game_rules());
    let state = game.check_game_status();
    assert_eq!(state, crate::game::domain::models::GameState::Draw);
}

#[test]
fn test_make_move_after_game_ended_restarts_game() {
    // Create a winning board for X (horizontal win in first row)
    let board = GameMap::load(vec![vec![FX, FX, FX], vec![FE, FO, FE], vec![FE, FE, FO]]);
    let mut game = Game::load("test_game".to_string(), board, O, default_game_rules());

    // Verify game has ended with X as winner
    let state = game.check_game_status();
    match state {
        crate::game::domain::models::GameState::Winner(p, _) => {
            assert_eq!(p, X);
        }
        _ => panic!("Expected X to win"),
    }

    // Make a move after game ended - should restart the game
    assert_eq!(game.make_move(1, 1), Ok(()));

    // Verify the board was cleared and restarted
    assert_eq!(game.board().get(1, 1), FO); // New move should be placed
    assert_eq!(game.board().get(0, 0), FE); // Old moves should be cleared
    assert_eq!(game.board().get(1, 0), FE);
    assert_eq!(game.board().get(2, 0), FE);

    // Verify current player switched after the move
    assert_eq!(game.current_player(), X);

    // Verify game is in progress again
    let new_state = game.check_game_status();
    assert_eq!(
        new_state,
        crate::game::domain::models::GameState::InProgress
    );
}

#[test]
fn test_make_move_after_draw_restarts_game() {
    // Board with a draw (all cells occupied, no winner)
    let board = GameMap::load(vec![vec![FX, FO, FX], vec![FO, FO, FX], vec![FO, FX, FO]]);
    let mut game = Game::load("test_game".to_string(), board, X, default_game_rules());

    // Verify game ended in a draw
    let state = game.check_game_status();
    assert_eq!(state, crate::game::domain::models::GameState::Draw);

    // Make a move after draw - should restart the game
    assert_eq!(game.make_move(0, 0), Ok(()));

    // Verify the board was cleared and restarted
    assert_eq!(game.board().get(0, 0), FO); // Current player is O after restart
    assert_eq!(game.board().get(0, 1), FE); // Other cells should be empty
    assert_eq!(game.board().get(1, 1), FE);

    // Verify current player switched after the move
    assert_eq!(game.current_player(), X);

    // Verify game is in progress again
    let new_state = game.check_game_status();
    assert_eq!(
        new_state,
        crate::game::domain::models::GameState::InProgress
    );
}

#[test]
fn test_win_takes_priority_over_draw() {
    // Board where all cells are occupied AND there's a winning condition
    // X wins horizontally in the first row, but the board is also full
    let board = GameMap::load(vec![vec![FX, FX, FX], vec![FO, FO, FX], vec![FO, FX, FO]]);
    let game = Game::load("test_game".to_string(), board, X, default_game_rules());
    let state = game.check_game_status();

    // Should detect win, not draw
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, X);
            assert_eq!(segment.direction(), SegmentDirection::Horizontal);
            assert_eq!(segment.start(), (0, 0));
            assert_eq!(segment.end(), (2, 0));
        }
        _ => panic!("Expected win for X, not draw, even though board is full"),
    }
}

#[test]
fn test_win_takes_priority_over_draw_diagonal() {
    // Board where all cells are occupied AND there's a winning condition
    // X wins diagonally (descending), but the board is also full
    // XOX
    // XXO
    // OOX
    let board = GameMap::load(vec![vec![FX, FO, FX], vec![FX, FX, FO], vec![FO, FO, FX]]);
    let game = Game::load("test_game".to_string(), board, X, default_game_rules());
    let state = game.check_game_status();

    // Should detect win, not draw
    match state {
        crate::game::domain::models::GameState::Winner(p, segment) => {
            assert_eq!(p, X);
            assert_eq!(segment.direction(), SegmentDirection::DiagonalDescending);
            assert_eq!(segment.start(), (0, 0));
            assert_eq!(segment.end(), (2, 2));
        }
        _ => panic!("Expected diagonal win for X, not draw, even though board is full"),
    }
}

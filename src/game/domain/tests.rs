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

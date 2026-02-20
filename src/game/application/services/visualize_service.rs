use teloc::inject;

use crate::game::{
    application::interfaces::{Asset, AssetsAdapter},
    domain::{
        aggregates::Game,
        models::{FieldState, GameState, Player},
        value_objects::{Point, SegmentDirection},
    },
};

pub struct VisualizeService {
    assets: Box<dyn AssetsAdapter>,
}

#[inject]
impl VisualizeService {
    pub fn new(assets: Box<dyn AssetsAdapter>) -> Self {
        Self { assets }
    }
}

impl VisualizeService {
    pub fn visualize_current_player(&self, game: &Game) -> Result<Vec<u8>, ()> {
        let is_game_over = game.check_game_status() != GameState::InProgress;

        let asset = if is_game_over {
            "game_over"
        } else {
            match game.current_player() {
                Player::O => "O",
                Player::X => "X",
            }
        };

        match self.assets.get(asset) {
            Ok(Asset::Image(a)) => Ok(a),
            _ => Err(()),
        }
    }

    pub fn visualize_field(&self, game: &Game, field_coordinates: Point) -> Result<Vec<u8>, ()> {
        let (x, y) = field_coordinates;
        let field = game.board().get(x as usize, y as usize);
        let status = game.check_game_status();

        let postfix = match status {
            GameState::Winner(_, segment) if segment.contains(field_coordinates) => {
                match segment.direction() {
                    SegmentDirection::DiagonalAscending => "_da",
                    SegmentDirection::DiagonalDescending => "_dd",
                    SegmentDirection::Vertical => "_v",
                    SegmentDirection::Horizontal => "_h",
                }
            }
            _ => "",
        };

        let asset = match field {
            FieldState::Empty => "empty".to_owned(),
            FieldState::Occupied(Player::O) => format!("O{postfix}"),
            FieldState::Occupied(Player::X) => format!("X{postfix}"),
        };

        match self.assets.get(&asset) {
            Ok(Asset::Image(a)) => Ok(a),
            _ => Err(()),
        }
    }
}

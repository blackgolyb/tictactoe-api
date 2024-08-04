use crate::core::{
    config::load_config,
    types::{FieldId, GameResult, GameStatus, Room},
};

use crate::repositories::traits::GameRepositoryInterface;

use super::{
    traits::{GameServiceInterface, VisualizeGame},
    GameService, GameVisualizeService,
};

pub struct GamePlayService {
    visualizer: Box<dyn VisualizeGame>,
    game_service: Box<dyn GameServiceInterface>,
}

impl GamePlayService {
    pub fn new(repo: Box<dyn GameRepositoryInterface>) -> Self {
        let config = load_config();
        let visualizer: Box<dyn VisualizeGame> =
            Box::new(GameVisualizeService::new(config.assets.clone()));
        let game_service: Box<dyn GameServiceInterface> = Box::new(GameService::new(repo));

        Self {
            visualizer,
            game_service,
        }
    }

    pub fn get_field_image(&self, room: Room, field_id: FieldId) -> Vec<u8> {
        let (game, _, winners_field) = self.game_service.check_game(room);
        let field_status = game.board[field_id as usize].clone();

        self.visualizer
            .get_field_image(field_id, field_status, winners_field)
    }

    pub fn make_step(&self, room: Room, field_id: FieldId) -> GameResult<()> {
        self.game_service.make_step(room, field_id)
    }

    pub fn get_current_player_image(&self, room: Room) -> Vec<u8> {
        let (game, status, _) = self.game_service.check_game(room);

        match status {
            GameStatus::InProgress | GameStatus::NotStarted => {
                self.visualizer
                    .get_field_image(0, game.current_player, None)
            }
            _ => self.visualizer.get_game_over_image(),
        }
    }
}

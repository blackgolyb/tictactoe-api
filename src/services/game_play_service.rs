use crate::core::{
    config::{load_config, Config},
    types::{FieldId, Room},
};

use crate::repositories::{traits::GameRepositoryInterface, InMemoryGameRepository};

use super::{traits::VisualizeGame, GameVisualizeService};

pub struct GamePlayService {
    config: Config,
    repo: Box<dyn GameRepositoryInterface>,
    visualizer: Box<dyn VisualizeGame>,
    // game_service: Box<dyn GameServiceInterface>,
}

impl GamePlayService {
    fn new() -> Self {
        let config = load_config();
        let repo: Box<dyn GameRepositoryInterface> = Box::new(InMemoryGameRepository::new());
        let visualizer: Box<dyn VisualizeGame> =
            Box::new(GameVisualizeService::new(config.assets.clone()));
        // let game_service: Box<dyn GameServiceInterface> = Box::new(GameService::new(repo));

        Self {
            config,
            repo,
            visualizer,
            // game_service,
        }
    }

    fn get_field_image(&self, room: Room, field_id: FieldId) -> Vec<u8> {
        vec![1, 4, 7]
    }
}

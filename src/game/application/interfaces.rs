use crate::game::domain::{aggregates::Game, models::GameId};

pub trait GameRepository {
    fn get(&self, id: GameId) -> Option<Game>;
    fn get_by_name(&self, name: &str) -> Option<Game>;
    fn save(&self, game: &Game);
}

pub enum Asset {
    Image(Vec<u8>),
}

pub trait AssetsAdapter {
    fn get(&self, asset: &str) -> Result<Asset, ()>;
}

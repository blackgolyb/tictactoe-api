use crate::game::domain::aggregates::Game;

pub trait GameRepository {
    fn get_by_name(&self, name: &str) -> Option<Game>;
    fn save(&self, game: &Game);
}

pub enum Asset {
    Image(Vec<u8>),
}

pub trait AssetsAdapter {
    fn get(&self, asset: &str) -> Result<Asset, ()>;
    fn get_resized_image(&self, asset: &str, size: (u32, u32)) -> Result<Asset, ()>;
}

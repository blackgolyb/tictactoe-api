use crate::game::domain::aggregates::Game;

pub trait GameRepository {
    fn get_by_name(&self, name: &str) -> Option<Game>;
    fn save(&self, game: &Game);
}

pub enum Asset {
    Image(Vec<u8>),
}

#[derive(Clone, Copy)]
pub struct ImageSize {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub trait AssetsAdapter {
    fn get(&self, asset: &str) -> Result<Asset, ()>;
    fn get_resized_image(&self, asset: &str, size: ImageSize) -> Result<Asset, ()>;
}

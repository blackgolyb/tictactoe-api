pub(self) mod base;

mod m0001_create_game;
mod m0002_remove_game_id;

pub use base::MigrationInterface;

pub fn get_migrations() -> Vec<Box<dyn MigrationInterface>> {
    vec![
        Box::new(m0001_create_game::Migration),
        Box::new(m0002_remove_game_id::Migration),
    ]
}

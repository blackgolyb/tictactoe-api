pub(self) mod base;

mod m0001_create_game;

pub use base::MigrationInterface;

pub fn get_migrations() -> Vec<Box<dyn MigrationInterface>> {
    vec![Box::new(m0001_create_game::Migration)]
}

use std::path::PathBuf;

use crate::game::{
    application::{
        interfaces::{AssetsAdapter, GameRepository},
        services::visualize_service::VisualizeService,
    },
    infrastructure::{
        assets::AssetsLoaderFromDisk,
        db::{repositories::SqliteGameRepository, DBConnection},
    },
};

pub struct DIContainer {
    db_connection: DBConnection,
    assets_dir: PathBuf,
}

impl DIContainer {
    pub fn new(db_connection: DBConnection, assets_dir: PathBuf) -> Self {
        Self {
            db_connection,
            assets_dir,
        }
    }

    pub fn game_repository(&self) -> Box<dyn GameRepository> {
        Box::new(SqliteGameRepository::new(self.db_connection.clone()))
    }

    pub fn assets_adapter(&self) -> Box<dyn AssetsAdapter> {
        Box::new(AssetsLoaderFromDisk::new(self.assets_dir.clone()))
    }

    pub fn visualize_service(&self) -> VisualizeService {
        VisualizeService::new(self.assets_adapter())
    }
}

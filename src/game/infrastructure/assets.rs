use std::path::PathBuf;

use crate::game::application::interfaces::{Asset, AssetsAdapter};

pub struct AssetsLoaderFromDisk {
    assets_dir: PathBuf,
}

pub enum AssetsTypes {
    Image,
}

impl AssetsLoaderFromDisk {
    pub fn new(assets_dir: PathBuf) -> Self {
        Self { assets_dir }
    }

    fn registry(&self, asset: &str) -> Option<(AssetsTypes, &str)> {
        match asset {
            "empty" => Some((AssetsTypes::Image, "empty.png")),
            "O" => Some((AssetsTypes::Image, "O.png")),
            "X" => Some((AssetsTypes::Image, "X.png")),
            "X_v" => Some((AssetsTypes::Image, "X_v.png")),
            "X_h" => Some((AssetsTypes::Image, "X_h.png")),
            "X_dd" => Some((AssetsTypes::Image, "X_dd.png")),
            "X_da" => Some((AssetsTypes::Image, "X_da.png")),
            "O_v" => Some((AssetsTypes::Image, "O_v.png")),
            "O_h" => Some((AssetsTypes::Image, "O_h.png")),
            "O_dd" => Some((AssetsTypes::Image, "O_dd.png")),
            "O_da" => Some((AssetsTypes::Image, "O_da.png")),
            _ => None,
        }
    }

    fn read_image(path: PathBuf) -> Result<Vec<u8>, ()> {
        std::fs::read(path).or(Err(()))
    }
}

impl AssetsAdapter for AssetsLoaderFromDisk {
    fn get(&self, asset: &str) -> Result<Asset, ()> {
        let (asset_type, file_path) = self.registry(asset).ok_or(())?;
        let full_path = self.assets_dir.join(file_path);

        match asset_type {
            AssetsTypes::Image => {
                let data = Self::read_image(full_path)?;
                Ok(Asset::Image(data))
            }
        }
    }
}

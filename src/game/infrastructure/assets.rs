use std::{io::Cursor, path::PathBuf};

use crate::game::application::interfaces::{Asset, AssetsAdapter};

pub struct AssetsLoaderFromDisk {
    store: DiskAssetsStore,
    cache: DiskAssetsCache,
}

struct DiskAssetsStore {
    assets_dir: PathBuf,
}

struct DiskAssetsCache {
    cache_dir: PathBuf,
}

#[derive(Clone, Copy)]
enum AssetType {
    Image,
}

fn resize_image(img: Vec<u8>, size: (u32, u32)) -> Result<Vec<u8>, ()> {
    if size.0 == 0 || size.1 == 0 {
        return Err(());
    }

    let image = image::load_from_memory(&img).or(Err(()))?;
    let resized = image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);
    let mut output = Cursor::new(Vec::new());

    resized
        .write_to(&mut output, image::ImageFormat::Png)
        .or(Err(()))?;

    Ok(output.into_inner())
}

fn resize_asset_image(asset: Asset, size: (u32, u32)) -> Result<Asset, ()> {
    match asset {
        Asset::Image(img) => Ok(Asset::Image(resize_image(img, size)?)),
    }
}

fn read_image(path: PathBuf) -> Result<Vec<u8>, ()> {
    std::fs::read(path).or(Err(()))
}

fn read_asset(asset_type: AssetType, path: PathBuf) -> Result<Asset, ()> {
    match asset_type {
        AssetType::Image => {
            let data = read_image(path)?;
            Ok(Asset::Image(data))
        }
    }
}

impl AssetsLoaderFromDisk {
    pub fn new(assets_dir: PathBuf) -> Self {
        Self {
            cache: DiskAssetsCache::new(assets_dir.join("__cache__")),
            store: DiskAssetsStore::new(assets_dir),
        }
    }
}

impl DiskAssetsStore {
    fn new(assets_dir: PathBuf) -> Self {
        Self { assets_dir }
    }

    fn registry(asset: &str) -> Option<(AssetType, &'static str)> {
        match asset {
            "empty" => Some((AssetType::Image, "empty.png")),
            "game_over" => Some((AssetType::Image, "game_over.png")),
            "O" => Some((AssetType::Image, "O.png")),
            "X" => Some((AssetType::Image, "X.png")),
            "X_v" => Some((AssetType::Image, "X_v.png")),
            "X_h" => Some((AssetType::Image, "X_h.png")),
            "X_dd" => Some((AssetType::Image, "X_dd.png")),
            "X_da" => Some((AssetType::Image, "X_da.png")),
            "O_v" => Some((AssetType::Image, "O_v.png")),
            "O_h" => Some((AssetType::Image, "O_h.png")),
            "O_dd" => Some((AssetType::Image, "O_dd.png")),
            "O_da" => Some((AssetType::Image, "O_da.png")),
            _ => None,
        }
    }

    fn asset_type(&self, asset: &str) -> Result<AssetType, ()> {
        let (asset_type, _) = Self::registry(asset).ok_or(())?;
        Ok(asset_type)
    }

    fn get(&self, asset: &str) -> Result<Asset, ()> {
        let (asset_type, file_path) = Self::registry(asset).ok_or(())?;
        let full_path = self.assets_dir.join(file_path);
        read_asset(asset_type, full_path)
    }
}

impl DiskAssetsCache {
    fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    fn get_cache_path(&self, asset: &str, size: (u32, u32)) -> PathBuf {
        self.cache_dir.join(format!("{}_{}_{}", size.0, size.1, asset))
    }

    fn set(&self, asset_name: &str, size: (u32, u32), asset: &Asset) -> Result<(), ()> {
        let cache_path = self.get_cache_path(asset_name, size);

        std::fs::create_dir_all(&self.cache_dir).or(Err(()))?;
        match asset {
            Asset::Image(img) => std::fs::write(cache_path, img).or(Err(()))?,
        }

        Ok(())
    }

    fn get(
        &self,
        asset: &str,
        asset_type: AssetType,
        size: (u32, u32),
    ) -> Result<Option<Asset>, ()> {
        let path = self.get_cache_path(asset, size);
        if !path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_asset(asset_type, path)?))
    }
}

impl AssetsAdapter for AssetsLoaderFromDisk {
    fn get(&self, asset: &str) -> Result<Asset, ()> {
        self.store.get(asset)
    }

    fn get_resized_image(&self, asset: &str, size: (u32, u32)) -> Result<Asset, ()> {
        let asset_type = self.store.asset_type(asset)?;
        if !matches!(asset_type, AssetType::Image) {
            return Err(());
        }

        if let Some(cached_asset) = self.cache.get(asset, asset_type, size)? {
            return Ok(cached_asset);
        }

        let resized_asset = resize_asset_image(self.store.get(asset)?, size)?;
        self.cache.set(asset, size, &resized_asset)?;

        Ok(resized_asset)
    }
}

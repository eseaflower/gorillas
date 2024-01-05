use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::Deserialize;
use std::{io::BufReader, path::Path};
use thiserror::Error;

mod ldtk_schema;

pub use ldtk_schema::LdtkJson as Project;
pub use ldtk_schema::*;

pub fn load(path: impl AsRef<Path>) -> Result<Project, std::io::Error> {
    let reader = BufReader::new(std::fs::File::open(path)?);
    Ok(serde_json::from_reader::<_, Project>(reader)?)
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LdtkAsset {
    pub project: Project,
}

#[derive(Default)]
pub struct LdtkAssetLoader;

/// Possible errors that can be produced by [`LdtkAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LdtkAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [serde_json](serde_json::error) Error
    #[error("Could not deserialize JSON: {0}")]
    RonSpannedError(#[from] serde_json::error::Error),
}

impl AssetLoader for LdtkAssetLoader {
    type Asset = LdtkAsset;
    type Settings = ();
    type Error = LdtkAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = serde_json::from_slice(&bytes)?;
            Ok(LdtkAsset {
                project: custom_asset,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldtk() {
        let file = "assets/test.ldtk";
        let project = load(file).expect("failed to load ldtk file");

        project.levels.iter().for_each(|level| {
            println!("level: {}", level.identifier);
            level.layer_instances.iter().for_each(|layer| {
                layer.iter().for_each(|instance| {
                    println!("instance: {}", instance.identifier);

                    instance.grid_tiles.iter().for_each(|tile| {
                        println!("tile: {:?}", tile);
                    });
                });
            });
        });
    }
}

pub fn convert_coords(px: &[i64], grid_size: f32, level_size: Vec2) -> Vec2 {
    let center_x = px[0] as f32 + grid_size / 2.0;
    let center_y = px[1] as f32 + grid_size / 2.0;

    // Transform into our coordinate system
    let center_y = level_size.y - center_y;

    // Move the entire level to the center
    let center_x = center_x - level_size.x / 2.0;
    let center_y = center_y - level_size.y / 2.0;

    Vec2::new(center_x, center_y)
}

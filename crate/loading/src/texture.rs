//! Provides a function to load a PNG image as a texture.

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{PngFormat, Texture, TextureHandle};

/// Returns a `TextureHandle` to the image.
///
/// This function expects the image to be in PNG format.
///
/// # Parameters
///
/// * `path`: Path to the sprite sheet.
/// * `world`: `World` that stores resources.
pub fn load<N>(path: N, world: &World) -> TextureHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        path,
        PngFormat,
        Default::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

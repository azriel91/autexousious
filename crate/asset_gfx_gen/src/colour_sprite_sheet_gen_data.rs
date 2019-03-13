use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Read, ReadExpect},
    renderer::{SpriteSheet, Texture},
};
use derivative::Derivative;
use shred_derive::SystemData;

/// System data needed to load colour sprites.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ColourSpriteSheetGenData<'s> {
    /// Asset `Loader`.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `Texture` assets.
    #[derivative(Debug = "ignore")]
    pub texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
}

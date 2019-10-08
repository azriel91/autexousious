use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World, Write},
    renderer::{sprite::SpriteSheetHandle, SpriteSheet, Texture},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use slotmap::SecondaryMap;

/// `TextureLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct TextureLoadingResources<'s> {
    /// `Texture` assets.
    #[derivative(Debug = "ignore")]
    pub texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `SecondaryMap<AssetId, Vec<SpriteSheetHandle>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_sheet_handles: Write<'s, SecondaryMap<AssetId, Vec<SpriteSheetHandle>>>,
}

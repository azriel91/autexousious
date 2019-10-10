use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World, Write},
    renderer::{sprite::SpriteSheetHandle, SpriteSheet, Texture},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use slotmap::SecondaryMap;

use crate::SpritesDefinitionLoadingResourcesRead;

/// `TextureLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct TextureLoadingResources<'s> {
    /// `SpritesDefinitionLoadingResourcesRead`.
    #[derivative(Debug = "ignore")]
    pub sprites_definition_loading_resources_read: SpritesDefinitionLoadingResourcesRead<'s>,
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

/// `TextureLoadingResourcesRead`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct TextureLoadingResourcesRead<'s> {
    /// `Texture` assets.
    #[derivative(Debug = "ignore")]
    pub texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `SecondaryMap<AssetId, Vec<SpriteSheetHandle>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_sheet_handles: Read<'s, SecondaryMap<AssetId, Vec<SpriteSheetHandle>>>,
}

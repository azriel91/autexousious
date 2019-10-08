use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use slotmap::SecondaryMap;
use sprite_model::config::SpritesDefinition;

/// `SpritesDefinitionLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpritesDefinitionLoadingResources<'s> {
    /// `SpritesDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub sprites_definition_assets: Read<'s, AssetStorage<SpritesDefinition>>,
    /// `SecondaryMap<AssetId, Handle<SpritesDefinition>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprites_definition_handles:
        Read<'s, SecondaryMap<AssetId, Handle<SpritesDefinition>>>,
}

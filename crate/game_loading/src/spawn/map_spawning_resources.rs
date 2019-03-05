use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Read},
};
use derivative::Derivative;
use map_model::loaded::Map;
use sequence_model::loaded::ComponentSequences;
use shred_derive::SystemData;

/// Resources needed to spawn a map.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSpawningResources<'res> {
    /// `EntitiesRes` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'res>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub map_assets: Read<'res, AssetStorage<Map>>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'res, AssetStorage<ComponentSequences>>,
}

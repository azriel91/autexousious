use amethyst::{
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use sequence_model::loaded::AssetSequenceEndTransitions;

/// Resources used to spawn object entities.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSpawningResources<'s> {
    /// `AssetSequenceEndTransitions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_end_transitions: Read<'s, AssetSequenceEndTransitions>,
}

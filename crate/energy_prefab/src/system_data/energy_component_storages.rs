use amethyst::{
    ecs::{World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use collision_model::loaded::{HitTransition, HittingTransition};
use derivative::Derivative;
use map_model::play::MapUnboundedDelete;
use object_model::play::Grounding;

/// Energy specific `Component` storages.
///
/// These are the storages for the components specific to energy objects. See also
/// `ObjectComponentStorages`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct EnergyComponentStorages<'s> {
    /// `Grounding` component storage.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `MapUnboundedDelete` component storage.
    #[derivative(Debug = "ignore")]
    pub map_unbounded_deletes: WriteStorage<'s, MapUnboundedDelete>,
    /// `HitTransition` components.
    #[derivative(Debug = "ignore")]
    pub hit_transitions: WriteStorage<'s, HitTransition>,
    /// `HittingTransition` components.
    #[derivative(Debug = "ignore")]
    pub hitting_transitions: WriteStorage<'s, HittingTransition>,
}

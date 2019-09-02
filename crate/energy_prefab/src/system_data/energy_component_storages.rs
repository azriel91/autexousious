use amethyst::{
    ecs::{World, WriteStorage},
    shred::{ResourceId, SystemData},
};
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
}

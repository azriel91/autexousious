use amethyst::ecs::WriteStorage;
use derivative::Derivative;
use object_model::play::{Grounding, HealthPoints};
use shred_derive::SystemData;

/// Energy specific `Component` storages.
///
/// These are the storages for the components specific to energy objects. See also
/// `ObjectComponentStorages`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct EnergyComponentStorages<'s> {
    /// `HealthPoints` component storage.
    #[derivative(Debug = "ignore")]
    pub health_pointses: WriteStorage<'s, HealthPoints>,
    /// `Grounding` component storage.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

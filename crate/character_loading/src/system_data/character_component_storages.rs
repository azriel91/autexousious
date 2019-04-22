use amethyst::ecs::WriteStorage;
use character_model::play::RunCounter;
use derivative::Derivative;
use game_input::ControllerInput;
use object_model::play::{Grounding, HealthPoints};
use shred_derive::SystemData;

/// Character specific `Component` storages.
///
/// These are the storages for the components specific to character objects. See also
/// `ObjectComponentStorages`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterComponentStorages<'s> {
    /// `ControllerInput` component storage.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: WriteStorage<'s, ControllerInput>,
    /// `HealthPoints` component storage.
    #[derivative(Debug = "ignore")]
    pub health_pointses: WriteStorage<'s, HealthPoints>,
    /// `RunCounter` component storage.
    #[derivative(Debug = "ignore")]
    pub run_counters: WriteStorage<'s, RunCounter>,
    /// `Grounding` component storage.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

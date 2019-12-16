use amethyst::{
    ecs::{ReadStorage, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use game_input::ControllerInput;
use mirrored_model::play::Mirrored;

/// `SystemData` used to determine if an input reaction's requirement is met.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct BasicIrrSystemData<'s> {
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
}

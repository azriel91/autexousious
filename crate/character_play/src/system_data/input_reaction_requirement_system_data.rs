use amethyst::{
    ecs::{ReadStorage, World},
    shred::{ResourceId, SystemData},
};
use charge_model::{config::ChargeUseMode, play::ChargeTrackerClock};
use derivative::Derivative;
use game_input::ControllerInput;
use mirrored_model::play::Mirrored;
use object_model::play::{HealthPoints, SkillPoints};

/// `SystemData` used to determine if an input reaction's requirement is met.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputReactionRequirementSystemData<'s> {
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: ReadStorage<'s, HealthPoints>,
    /// `SkillPoints` components.
    #[derivative(Debug = "ignore")]
    pub skill_pointses: ReadStorage<'s, SkillPoints>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: ReadStorage<'s, ChargeTrackerClock>,
    /// `ChargeUseMode` components.
    #[derivative(Debug = "ignore")]
    pub charge_use_modes: ReadStorage<'s, ChargeUseMode>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
}

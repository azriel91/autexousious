use amethyst::ecs::ReadStorage;
use charge_model::{config::ChargeUseMode, play::ChargeTrackerClock};
use derivative::Derivative;
use game_input::ControllerInput;
use object_model::play::{HealthPoints, Mirrored, SkillPoints};
use shred_derive::SystemData;

/// `SystemData` used to determine if a transition's requirement is met.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ControlTransitionRequirementSystemData<'s> {
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

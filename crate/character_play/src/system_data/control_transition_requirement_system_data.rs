use amethyst::ecs::ReadStorage;
use derivative::Derivative;
use object_model::play::{ChargePoints, HealthPoints, SkillPoints};
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
    /// `ChargePoints` components.
    #[derivative(Debug = "ignore")]
    pub charge_pointses: ReadStorage<'s, ChargePoints>,
}

use amethyst::{ecs::Entity, shred::SystemData};

/// Conditions to be met for an input reaction to happen.
pub trait InputReactionRequirement<'s> {
    /// `SystemData` to query to check if requirement is met.
    type SystemData: SystemData<'s>;

    /// Whether the requirement is met.
    fn requirement_met(&self, system_data: &mut Self::SystemData, entity: Entity) -> bool;
}

impl<'s> InputReactionRequirement<'s> for () {
    type SystemData = ();

    fn requirement_met(&self, _: &mut Self::SystemData, _: Entity) -> bool {
        true
    }
}

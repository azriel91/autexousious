use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use typename_derive::TypeName;

/// State that tracks an object's attachment to the surrounding environment.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq, TypeName)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum Grounding {
    /// Object is in the air.
    #[derivative(Default)]
    Airborne,
    /// Object is resting on the ground, whether it is the floor or another solid object.
    OnGround,
    /// Object is below ground.
    Underground,
}

/// `GroundingSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GroundingSystemData<'s> {
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> ItemComponent<'s> for Grounding {
    type SystemData = GroundingSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let GroundingSystemData { groundings } = system_data;

        if groundings.get(entity).is_none() {
            groundings
                .insert(entity, *self)
                .expect("Failed to insert `Grounding` component.");
        }
    }
}

use amethyst::ecs::{
    shred::{ResourceId, SystemData},
    storage::HashMapStorage,
    Component, Entity, World, WriteStorage,
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use game_input_model::ControllerId;

/// Marks a game input controlled entity.
///
/// Stores the controller ID.
///
/// We use a `HashMapStorage` because there wouldn't be that many entities that are controlled by
/// `Controller`s. We will use a different `Component` for AI controllers.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(HashMapStorage)]
pub struct InputControlled {
    /// ID of the controller that controls the entity.
    pub controller_id: ControllerId,
}

/// `InputControlledSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputControlledSystemData<'s> {
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
}

impl<'s> ItemComponent<'s> for InputControlled {
    type SystemData = InputControlledSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let InputControlledSystemData { input_controlleds } = system_data;

        if input_controlleds.get(entity).is_none() {
            input_controlleds
                .insert(entity, *self)
                .expect("Failed to insert `InputControlled` component.");
        }
    }
}

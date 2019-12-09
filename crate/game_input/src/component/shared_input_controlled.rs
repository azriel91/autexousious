use amethyst::ecs::{
    shred::{ResourceId, SystemData},
    storage::NullStorage,
    Component, Entity, World, WriteStorage,
};
use asset_model::ItemComponent;
use derivative::Derivative;
use typename_derive::TypeName;

/// Marks an entity that responds to controls from all controllers.
///
/// We use a `NullStorage` because this is simply a tag on an entity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TypeName)]
pub struct SharedInputControlled;

impl Component for SharedInputControlled {
    type Storage = NullStorage<Self>;
}

/// `SharedInputControlledSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SharedInputControlledSystemData<'s> {
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: WriteStorage<'s, SharedInputControlled>,
}

impl<'s> ItemComponent<'s> for SharedInputControlled {
    type SystemData = SharedInputControlledSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SharedInputControlledSystemData {
            shared_input_controlleds,
        } = system_data;

        if shared_input_controlleds.get(entity).is_none() {
            shared_input_controlleds
                .insert(entity, *self)
                .expect("Failed to insert `SharedInputControlled` component.");
        }
    }
}

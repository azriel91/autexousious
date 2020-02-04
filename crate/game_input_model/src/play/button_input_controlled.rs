use amethyst::ecs::{
    shred::{ResourceId, SystemData},
    storage::NullStorage,
    Component, Entity, World, WriteStorage,
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Marks an entity that responds to controls from any device button.
///
/// We use a `NullStorage` because this is simply a tag on an entity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonInputControlled;

impl Component for ButtonInputControlled {
    type Storage = NullStorage<Self>;
}

/// `ButtonInputControlledSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ButtonInputControlledSystemData<'s> {
    /// `ButtonInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub button_input_controlleds: WriteStorage<'s, ButtonInputControlled>,
}

impl<'s> ItemComponent<'s> for ButtonInputControlled {
    type SystemData = ButtonInputControlledSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let ButtonInputControlledSystemData {
            button_input_controlleds,
        } = system_data;

        if button_input_controlleds.get(entity).is_none() {
            button_input_controlleds
                .insert(entity, *self)
                .expect("Failed to insert `ButtonInputControlled` component.");
        }
    }
}

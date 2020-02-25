use amethyst::{
    ecs::{
        shred::{ResourceId, SystemData},
        storage::VecStorage,
        Component, Entity, World, WriteStorage,
    },
    ui::{Interactable, Selectable},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Marks an entity that responds to regular keyboard / mouse input.
///
/// These are typically for UIs that are not designed to be interacted with controller input.
///
/// TODO: Should this and `ButtonInputControlled` be merged?
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NormalInputControlled {
    /// Tab order for widgets.
    pub tab_order: u32,
}

impl Component for NormalInputControlled {
    type Storage = VecStorage<Self>;
}

/// `NormalInputControlledSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NormalInputControlledSystemData<'s> {
    /// `NormalInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub normal_input_controlleds: WriteStorage<'s, NormalInputControlled>,
    /// `Selectable<()>` components.
    #[derivative(Debug = "ignore")]
    pub selectables: WriteStorage<'s, Selectable<()>>,
    /// `Interactable` components.
    #[derivative(Debug = "ignore")]
    pub interactables: WriteStorage<'s, Interactable>,
}

impl<'s> ItemComponent<'s> for NormalInputControlled {
    type SystemData = NormalInputControlledSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let NormalInputControlledSystemData {
            normal_input_controlleds,
            selectables,
            interactables,
        } = system_data;

        if normal_input_controlleds.get(entity).is_none() {
            normal_input_controlleds
                .insert(entity, *self)
                .expect("Failed to insert `NormalInputControlled` component.");
        }
        if selectables.get(entity).is_none() {
            selectables
                .insert(entity, Selectable::<()>::new(self.tab_order))
                .expect("Failed to insert `Selectable<()>` component.");
        }
        if interactables.get(entity).is_none() {
            interactables
                .insert(entity, Interactable)
                .expect("Failed to insert `Interactable` component.");
        }
    }
}

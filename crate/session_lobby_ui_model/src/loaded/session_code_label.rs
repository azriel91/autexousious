use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Marks entities that should display the `SessionCode`.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct SessionCodeLabel;

/// `SessionCodeLabelSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionCodeLabelSystemData<'s> {
    /// `SessionCodeLabel` components.
    #[derivative(Debug = "ignore")]
    pub session_devices_widgets: WriteStorage<'s, SessionCodeLabel>,
}

impl<'s> ItemComponent<'s> for SessionCodeLabel {
    type SystemData = SessionCodeLabelSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SessionCodeLabelSystemData {
            session_devices_widgets,
        } = system_data;

        if session_devices_widgets.get(entity).is_none() {
            session_devices_widgets
                .insert(entity, SessionCodeLabel)
                .expect("Failed to insert `SessionCodeLabel` component.");
        }
    }
}

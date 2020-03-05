use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Marks the `SessionDevicesWidget` entity.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct SessionDevicesWidget;

/// `SessionDevicesWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionDevicesWidgetSystemData<'s> {
    /// `SessionDevicesWidget` components.
    #[derivative(Debug = "ignore")]
    pub session_devices_widgets: WriteStorage<'s, SessionDevicesWidget>,
}

impl<'s> ItemComponent<'s> for SessionDevicesWidget {
    type SystemData = SessionDevicesWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SessionDevicesWidgetSystemData {
            session_devices_widgets,
        } = system_data;

        if session_devices_widgets.get(entity).is_none() {
            session_devices_widgets
                .insert(entity, SessionDevicesWidget)
                .expect("Failed to insert `SessionDevicesWidget` component.");
        }
    }
}

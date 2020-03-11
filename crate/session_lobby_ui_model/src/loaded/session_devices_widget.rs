use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use derivative::Derivative;
use derive_new::new;

use crate::play::SessionDevicesEntities;

/// Marks the `SessionDevicesWidget` entity.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(DenseVecStorage)]
pub struct SessionDevicesWidget {
    /// `ItemId` for entities that display a `SessionDeviceId`.
    pub item_id_session_device_id: ItemId,
    /// `ItemId` for entities that display a `SessionDeviceName`.
    pub item_id_session_device_name: ItemId,
}

/// `SessionDevicesWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionDevicesWidgetSystemData<'s> {
    /// `SessionDevicesEntities` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices_entities: Write<'s, SessionDevicesEntities>,
    /// `SessionDevicesWidget` components.
    #[derivative(Debug = "ignore")]
    pub session_devices_widgets: WriteStorage<'s, SessionDevicesWidget>,
}

impl<'s> ItemComponent<'s> for SessionDevicesWidget {
    type SystemData = SessionDevicesWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SessionDevicesWidgetSystemData {
            session_devices_entities,
            session_devices_widgets,
        } = system_data;

        session_devices_entities.session_devices_entity = Some(entity);
        session_devices_entities.session_device_entities.clear();

        if !session_devices_widgets.contains(entity) {
            session_devices_widgets
                .insert(entity, *self)
                .expect("Failed to insert `SessionDevicesWidget` component.");
        }
    }
}

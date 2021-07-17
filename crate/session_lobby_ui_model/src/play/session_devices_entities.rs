use amethyst::ecs::Entity;
use derive_new::new;

/// Entities of the `SessionDevicesWidget`.
///
/// This is used to track the main widget entity, as well as each
/// `SessionDeviceWidget` entity.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct SessionDevicesEntities {
    /// Main `SessionDevicesWidget` entity.
    pub session_devices_entity: Option<Entity>,
    /// Entities for each session device widget.
    pub session_device_entities: Vec<Entity>,
}

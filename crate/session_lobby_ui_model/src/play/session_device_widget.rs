use amethyst::ecs::{storage::DenseVecStorage, Component, Entity};
use derive_new::new;

/// Entities that make up a `SessionDeviceWidget`.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct SessionDeviceWidget {
    /// Entity for the session device name.
    pub entity_id: Entity,
    /// Entity for the session device ID.
    pub entity_name: Entity,
}

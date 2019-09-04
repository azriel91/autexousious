use amethyst::ecs::{storage::NullStorage, Component};
use specs_derive::Component;

/// Marks an entity as tracked by the game camera.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct CameraTracked;

use amethyst::ecs::{storage::NullStorage, Component};

/// Marks an entity as an asset preview widget preview entity.
///
/// Previous preview entities should be deleted, and new preview entities should be spawned when the
/// character selection is switched.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct ApwPreview;

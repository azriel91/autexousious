use amethyst::ecs::{storage::NullStorage, Component};

/// Marks an entity as a map selection widget preview entity.
///
/// Previous preview entities should be deleted, and new preview entities should be spawned when the
/// map selection is switched.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct MswPreview;

use amethyst::ecs::{storage::NullStorage, Component};

/// ID tag for entities created in the `NetworkModeSelectionState`.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct NetworkModeSelectionEntity;

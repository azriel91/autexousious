use amethyst::ecs::{storage::NullStorage, Component};

/// Indicates the Z position should be rendered as part of the Y transform.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PositionZAsY;

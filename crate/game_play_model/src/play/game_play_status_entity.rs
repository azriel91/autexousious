use amethyst::ecs::{storage::NullStorage, Component};

/// Marker for entities that display the game play status.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct GamePlayStatusEntity;

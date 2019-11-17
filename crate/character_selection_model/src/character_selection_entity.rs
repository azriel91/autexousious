use amethyst::ecs::{storage::NullStorage, Component};

/// ID tag for entities created in the `CharacterSelectionState`.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct CharacterSelectionEntity;

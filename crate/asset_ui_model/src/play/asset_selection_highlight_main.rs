use amethyst::ecs::{storage::NullStorage, Component};

/// Marks an entity as the main `AssetSelectionHighlightMain` entity.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct AssetSelectionHighlightMain;

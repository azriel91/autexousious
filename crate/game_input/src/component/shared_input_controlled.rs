use amethyst::ecs::{prelude::*, storage::NullStorage};

/// Marks an entity that responds to controls from all controllers.
///
/// We use a `NullStorage` because this is simply a tag on an entity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SharedInputControlled;

impl Component for SharedInputControlled {
    type Storage = NullStorage<Self>;
}

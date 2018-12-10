use amethyst::ecs::{prelude::*, storage::HashMapStorage};
use derive_new::new;

use crate::ControllerId;

/// Marks a game input controlled entity.
///
/// Stores the controller ID.
///
/// We use a `HashMapStorage` because there wouldn't be that many entities that are controlled by
/// `Controller`s. We will use a different `Component` for AI controllers.
#[derive(Debug, new)]
pub struct InputControlled {
    /// ID of the controller that controls the entity.
    pub controller_id: ControllerId,
}

impl Component for InputControlled {
    type Storage = HashMapStorage<Self>;
}

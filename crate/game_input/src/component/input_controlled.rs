use amethyst::ecs::{storage::HashMapStorage, Component};
use derive_new::new;
use game_input_model::ControllerId;

/// Marks a game input controlled entity.
///
/// Stores the controller ID.
///
/// We use a `HashMapStorage` because there wouldn't be that many entities that are controlled by
/// `Controller`s. We will use a different `Component` for AI controllers.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(HashMapStorage)]
pub struct InputControlled {
    /// ID of the controller that controls the entity.
    pub controller_id: ControllerId,
}

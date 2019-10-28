use amethyst::ecs::Entity;
use derive_new::new;

use crate::StateId;

/// `StateId` and entities used for the State UI.
#[derive(Clone, Debug, PartialEq, new)]
pub struct StateUiData {
    /// State ID that the current entities are built for.
    pub state_id: StateId,
    /// `Entities` spawned for state UI.
    pub entities: Vec<Entity>,
}

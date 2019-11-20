use amethyst::ecs::Entity;
use derive_new::new;

use crate::StateId;

/// `StateId` and entities used for `State` items.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct StateItemEntities {
    /// State ID that the current entities are built for.
    pub state_id: StateId,
    /// `Entities` spawned based on the state's asset `ItemIds`.
    pub entities: Vec<Entity>,
}

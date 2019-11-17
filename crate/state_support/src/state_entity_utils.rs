use amethyst::ecs::{world::EntitiesRes, Component, Join, World, WorldExt};

/// Functions to query `EntityId`s for `State`s.
#[derive(Debug)]
pub struct StateEntityUtils;

impl StateEntityUtils {
    /// Deletes all entities with the given component.
    ///
    /// **Note:** This does not call `world.maintain()`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` that the entities reside in.
    ///
    /// # Type Parameters
    ///
    /// * `I`: Component attached to entities that should be deleted.
    pub fn clear<I>(world: &mut World)
    where
        I: Component,
        I::Storage: Default,
    {
        let entities = world.read_resource::<EntitiesRes>();
        let state_tags = world.read_storage::<I>();
        (&entities, &state_tags).join().for_each(|(entity, _)| {
            entities
                .delete(entity)
                .expect("Failed to delete state entity.")
        });
    }
}

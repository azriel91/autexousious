use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use parent_model::play::ParentEntity;

/// Deletes entities whose `ParentEntity` is dead.
#[derive(Debug, Default, new)]
pub struct ChildEntityDeleteSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChildEntityDeleteSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: ReadStorage<'s, ParentEntity>,
}

impl<'s> System<'s> for ChildEntityDeleteSystem {
    type SystemData = ChildEntityDeleteSystemData<'s>;

    fn run(
        &mut self,
        ChildEntityDeleteSystemData {
            entities,
            parent_entities,
        }: Self::SystemData,
    ) {
        (&entities, &parent_entities)
            .join()
            .for_each(|(entity_child, parent_entity)| {
                if !entities.is_alive(parent_entity.0) {
                    if let Err(e) = entities.delete(entity_child) {
                        error!("Failed to delete entity: {}", e);
                    }
                }
            });
    }
}

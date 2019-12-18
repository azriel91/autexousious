#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entities, Entity, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use parent_model::play::ParentEntity;

    use parent_play::ChildEntityDeleteSystem;

    #[test]
    fn deletes_entities_when_parent_entity_is_dead() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChildEntityDeleteSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_parent = world.create_entity().build();
                let entity_child = world
                    .create_entity()
                    .with(ParentEntity::new(entity_parent))
                    .build();

                world.insert((entity_parent, entity_child));
            })
            .with_assertion(|world| {
                let (entity_parent, entity_child) = *world.read_resource::<(Entity, Entity)>();
                let entities = world.system_data::<Entities<'_>>();

                assert!(entities.is_alive(entity_parent));
                assert!(entities.is_alive(entity_child));
            })
            .with_effect(|world| {
                let (entity_parent, _entity_child) = *world.read_resource::<(Entity, Entity)>();
                let entities = world.system_data::<Entities<'_>>();

                entities
                    .delete(entity_parent)
                    .expect("Failed to delete `entity_parent`.");
            })
            .with_effect(|_| {}) // Wait for one more tick.
            .with_assertion(|world| {
                let (entity_parent, entity_child) = *world.read_resource::<(Entity, Entity)>();
                let entities = world.system_data::<Entities<'_>>();

                assert!(!entities.is_alive(entity_parent));
                assert!(!entities.is_alive(entity_child));
            })
            .run()
    }

    #[test]
    fn ignores_entities_without_parent_entity_component() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChildEntityDeleteSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_parent = world.create_entity().build();
                let entity_other = world.create_entity().build();

                world.insert((entity_parent, entity_other));
            })
            .with_effect(|world| {
                let (entity_parent, _entity_other) = *world.read_resource::<(Entity, Entity)>();
                let entities = world.system_data::<Entities<'_>>();

                entities
                    .delete(entity_parent)
                    .expect("Failed to delete `entity_parent`.");
            })
            .with_effect(|_| {}) // Wait for one more tick.
            .with_assertion(|world| {
                let (entity_parent, entity_other) = *world.read_resource::<(Entity, Entity)>();
                let entities = world.system_data::<Entities<'_>>();

                assert!(!entities.is_alive(entity_parent));
                assert!(entities.is_alive(entity_other));
            })
            .run()
    }
}

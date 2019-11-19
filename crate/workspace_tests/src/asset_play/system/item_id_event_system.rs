#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::{loaded::ItemId, play::ItemIdEvent};

    use asset_play::ItemIdEventSystem;

    #[test]
    fn attaches_handle_and_sends_event_for_item_id_insertions() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, None),
            |world| {
                let item_entity = world.create_entity().build();
                let item_id = ItemId::new(item_entity);

                insert_item_id(world, item_id);

                world.insert(item_id);
            },
            |world| {
                let item_id_expected = *world.read_resource::<ItemId>();
                create_or_update_events(world, item_id_expected)
            },
        )
    }

    #[test]
    fn attaches_handle_and_sends_event_for_item_id_modifications() -> Result<(), Error> {
        run_test(
            |world| {
                let item_entity = world.create_entity().build();
                let item_id = ItemId::new(item_entity);
                create_entity(world, Some(item_id))
            },
            |world| {
                let item_entity = world.create_entity().build();
                let item_id = ItemId::new(item_entity);

                update_item_id(world, item_id);

                world.insert(item_id);
            },
            |world| {
                let item_id_expected = *world.read_resource::<ItemId>();
                create_or_update_events(world, item_id_expected)
            },
        )
    }

    fn run_test(
        entity_create_fn: fn(&mut World),
        item_id_alter_fn: fn(&mut World),
        item_id_events_expected_fn: fn(&mut World) -> Vec<ItemIdEvent>,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(ItemIdEventSystem::new(), "", &[])
            .with_effect(entity_create_fn)
            .with_effect(register_reader)
            .with_effect(item_id_alter_fn)
            .with_assertion(move |world| {
                let events_expected = item_id_events_expected_fn(world);
                expect_events(world, events_expected);
            })
            .run_isolated()
    }

    fn register_reader(world: &mut World) {
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<ItemIdEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn insert_item_id(world: &mut World, item_id: ItemId) {
        let entity = *world.read_resource::<Entity>();

        let mut item_ids = world.write_storage::<ItemId>();
        item_ids
            .insert(entity, item_id)
            .expect("Failed to insert `ItemId`.");
    }

    fn update_item_id(world: &mut World, item_id: ItemId) {
        let entity = *world.read_resource::<Entity>();

        let mut item_ids = world.write_storage::<ItemId>();
        let item_id_existing = item_ids
            .get_mut(entity)
            .expect("Expected entity to contain `ItemId` component.");
        *item_id_existing = item_id;
    }

    fn create_entity(world: &mut World, item_id: Option<ItemId>) {
        let mut entity_builder = world.create_entity();
        if let Some(item_id) = item_id {
            entity_builder = entity_builder.with(item_id);
        }
        let entity = entity_builder.build();

        world.insert(entity);
    }

    fn create_or_update_events(world: &mut World, item_id: ItemId) -> Vec<ItemIdEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![ItemIdEvent::CreateOrUpdate { entity, item_id }]
    }

    fn expect_events(world: &mut World, events_expected: Vec<ItemIdEvent>) {
        let target_entity = *world.read_resource::<Entity>();
        let mut reader_id = world.write_resource::<ReaderId<ItemIdEvent>>();
        let ec = world.read_resource::<EventChannel<ItemIdEvent>>();

        // Filter events for the entity we care about.
        let events_actual = ec
            .read(&mut reader_id)
            .filter(|ev| match ev {
                ItemIdEvent::CreateOrUpdate { entity, .. } => target_entity == *entity,
            })
            .copied()
            .collect::<Vec<ItemIdEvent>>();

        assert_eq!(events_expected, events_actual)
    }
}

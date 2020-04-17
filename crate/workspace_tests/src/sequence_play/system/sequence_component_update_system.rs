#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication, SequenceQueries};
    use asset_model::loaded::AssetItemIds;
    use assets_test::CHAR_BAT_SLUG;
    use sequence_model::{
        loaded::{SequenceId, WaitSequenceHandle, WaitSequenceHandles},
        play::SequenceUpdateEvent,
    };

    use sequence_play::SequenceComponentUpdateSystem;

    const SEQUENCE_ID_PREV: SequenceId = SequenceId(1);
    const SEQUENCE_ID_CURRENT: SequenceId = SequenceId(2);

    #[test]
    fn updates_sequence_component_on_sequence_begin_event() -> Result<(), Error> {
        run_test(sequence_begin_events, true, SEQUENCE_ID_CURRENT)
    }

    #[test]
    fn does_not_update_sequence_component_on_frame_begin_event() -> Result<(), Error> {
        run_test(frame_begin_events, true, SEQUENCE_ID_PREV)
    }

    #[test]
    fn does_not_panic_when_entity_does_not_have_asset_id() -> Result<(), Error> {
        run_test(sequence_begin_events, false, SEQUENCE_ID_PREV)
    }

    fn run_test(
        sequence_update_events_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
        with_item_id: bool,
        sequence_id_expected: SequenceId,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(
                SequenceComponentUpdateSystem::<WaitSequenceHandles>::new(),
                "",
                &[],
            )
            .with_effect(move |world| initial_values(world, with_item_id))
            .with_effect(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(move |world| {
                let wait_sequence_handle_expected = SequenceQueries::wait_sequence_handle(
                    world,
                    &*CHAR_BAT_SLUG,
                    sequence_id_expected,
                );
                expect_component_values(world, wait_sequence_handle_expected)
            })
            .run_winit_loop()
    }

    fn initial_values(world: &mut World, with_item_id: bool) {
        let entity = {
            let wait_sequence_handle =
                SequenceQueries::wait_sequence_handle(world, &*CHAR_BAT_SLUG, SEQUENCE_ID_PREV);

            let item_id = if with_item_id {
                let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
                let item_id = {
                    let asset_item_ids = world.read_resource::<AssetItemIds>();
                    let item_ids = asset_item_ids
                        .get(asset_id)
                        .expect("Expected `ItemIds` to exist.");
                    item_ids
                        .first()
                        .copied()
                        .expect("Expected at least one `ItemId` to exist.")
                };
                Some(item_id)
            } else {
                None
            };

            let mut entity_builder = world
                .create_entity()
                .with(SEQUENCE_ID_CURRENT)
                .with(wait_sequence_handle);

            if let Some(item_id) = item_id {
                entity_builder = entity_builder.with(item_id);
            }

            entity_builder.build()
        };

        world.insert(entity);
    }

    fn expect_component_values(
        world: &mut World,
        wait_sequence_handle_expected: WaitSequenceHandle,
    ) {
        let entity = *world.read_resource::<Entity>();
        let wait_sequence_handles = world.read_storage::<WaitSequenceHandle>();

        let wait_sequence_handle_actual = wait_sequence_handles
            .get(entity)
            .expect("Expected entity to have `WaitSequenceHandle` component.");
        assert_eq!(&wait_sequence_handle_expected, wait_sequence_handle_actual);
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SEQUENCE_ID_CURRENT,
        }]
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index: 0,
        }]
    }
}

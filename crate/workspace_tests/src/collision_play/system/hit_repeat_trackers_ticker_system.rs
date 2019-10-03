#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::play::{HitRepeatClock, HitRepeatTracker, HitRepeatTrackers};
    use slotmap::SlotMap;

    use collision_play::HitRepeatTrackersTickerSystem;

    const ENTITY_TO_COUNT: usize = 3;
    const CLOCK_LIMIT: usize = 4;

    #[test]
    fn ticks_every_clock() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersTickerSystem::new(), "", &[])
            .with_effect(create_entities_with_hit_repeat_trackers)
            .with_assertion(|world| assert_clock_values(world, 1))
            .with_assertion(|world| {
                assert_clock_values(world, 2);

                let (entity_from_0, entity_from_1) = *world.read_resource::<(Entity, Entity)>();
                assert_trackers_count(world, entity_from_0, ENTITY_TO_COUNT);
                assert_trackers_count(world, entity_from_1, ENTITY_TO_COUNT);
            })
            .run()
    }

    #[test]
    fn deletes_completed_clocks() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersTickerSystem::new(), "", &[])
            .with_effect(create_entities_with_hit_repeat_trackers)
            .with_effect(|world| {
                let (entity_from_0, _entity_from_1) = *world.read_resource::<(Entity, Entity)>();

                let mut hit_repeat_trackerses = world.write_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses
                    .get_mut(entity_from_0)
                    .expect("Expected `HitRepeatTrackers` component to exist.");
                (*(*hit_repeat_trackers)
                    .values_mut()
                    .next()
                    .expect("Expected `HitRepeatTracker` to exist.")
                    .clock)
                    .value = CLOCK_LIMIT;
            })
            .with_assertion(|world| assert_clock_values(world, 2))
            .with_assertion(|world| {
                assert_clock_values(world, 3);

                let (entity_from_0, entity_from_1) = *world.read_resource::<(Entity, Entity)>();
                assert_trackers_count(world, entity_from_0, ENTITY_TO_COUNT - 1);
                assert_trackers_count(world, entity_from_1, ENTITY_TO_COUNT);
            })
            .with_assertion(|world| {
                let (entity_from_0, entity_from_1) = *world.read_resource::<(Entity, Entity)>();
                assert_trackers_count(world, entity_from_0, 0);
                assert_trackers_count(world, entity_from_1, 0);
            })
            .run()
    }

    fn create_entities_with_hit_repeat_trackers(world: &mut World) {
        let entity_tos = (0..ENTITY_TO_COUNT)
            .map(|_| world.create_entity().build())
            .collect::<Vec<Entity>>();
        let hit_repeat_trackers = hit_repeat_trackers(entity_tos);

        let entity_from_0 = world.create_entity().build();
        let entity_from_1 = world.create_entity().build();
        {
            let mut hit_repeat_trackerses = world.write_storage::<HitRepeatTrackers>();
            hit_repeat_trackerses
                .insert(entity_from_0, hit_repeat_trackers.clone())
                .expect("Failed to insert `HitRepeatTrackers` component.");
            hit_repeat_trackerses
                .insert(entity_from_1, hit_repeat_trackers)
                .expect("Failed to insert `HitRepeatTrackers` component.");
        }

        world.insert((entity_from_0, entity_from_1));
    }

    fn assert_clock_values(world: &mut World, expected_value: usize) {
        let (entity_from_0, entity_from_1) = *world.read_resource::<(Entity, Entity)>();
        let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();

        [entity_from_0, entity_from_1]
            .iter()
            .for_each(|entity_from| {
                let hit_repeat_trackers = hit_repeat_trackerses
                    .get(*entity_from)
                    .expect("Expected `HitRepeatTrackers` component to exist.");
                hit_repeat_trackers
                    .values()
                    .for_each(|HitRepeatTracker { clock, .. }| {
                        assert_eq!(expected_value, (*clock).value)
                    });
            })
    } // kcov-ignore

    fn assert_trackers_count(world: &mut World, entity_from: Entity, expected_len: usize) {
        let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
        let hit_repeat_trackers = hit_repeat_trackerses
            .get(entity_from)
            .expect("Expected `HitRepeatTrackers` component to exist.");

        assert_eq!(expected_len, hit_repeat_trackers.len());
    }

    fn hit_repeat_trackers(entities: Vec<Entity>) -> HitRepeatTrackers {
        let slot_map = entities.into_iter().map(hit_repeat_tracker).fold(
            SlotMap::new(),
            |mut slot_map, hit_repeat_tracker| {
                slot_map.insert(hit_repeat_tracker);
                slot_map
            },
        );
        HitRepeatTrackers::new(slot_map)
    }

    fn hit_repeat_tracker(entity_to: Entity) -> HitRepeatTracker {
        let hit_repeat_clock = HitRepeatClock::new(CLOCK_LIMIT);
        HitRepeatTracker::new(entity_to, hit_repeat_clock)
    }
}

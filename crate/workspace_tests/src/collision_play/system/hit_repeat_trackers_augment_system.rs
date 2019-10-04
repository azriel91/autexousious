#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind},
        play::{HitEvent, HitRepeatClock, HitRepeatTracker, HitRepeatTrackers},
    };
    use shape_model::Volume;
    use slotmap::SlotMap;

    use collision_play::HitRepeatTrackersAugmentSystem;

    #[test]
    fn inserts_hit_repeat_trackers_for_hitter() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                send_event(world, event);

                world.insert((entity_from, entity_to));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .run()
    }

    #[test]
    fn inserts_hit_repeat_tracker_for_different_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to_0 = world.create_entity().build();
                let entity_to_1 = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to_0, interaction(), body());
                send_event(world, event);

                world.insert((entity_from, entity_to_0, entity_to_1));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, _entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to_0, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .with_effect(|world| {
                let (entity_from, _entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();

                let event = HitEvent::new(entity_from, entity_to_1, interaction(), body());
                send_event(world, event);
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to_0, HitRepeatClock::new(4)));
                slot_map.insert(HitRepeatTracker::new(entity_to_1, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .run()
    }

    fn send_event(world: &mut World, event: HitEvent) {
        let mut ec = world.write_resource::<EventChannel<HitEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn interaction() -> Interaction {
        Interaction::new(
            InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(4),
                hit_limit: HitLimit::Unlimited,
                ..Default::default()
            }),
            vec![],
            true,
        )
    }

    fn body() -> Volume {
        Volume::Box {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
            h: 1,
            d: 1,
        }
    }
}

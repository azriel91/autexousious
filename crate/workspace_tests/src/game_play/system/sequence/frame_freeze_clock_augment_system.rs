#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, Interaction, InteractionKind},
        play::HitEvent,
    };
    use sequence_model::play::FrameFreezeClock;
    use shape_model::Volume;

    use game_play::FrameFreezeClockAugmentSystem;

    #[test]
    fn inserts_frame_freeze_clock_for_hitter() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(FrameFreezeClockAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                send_event(world, event);

                world.insert(entity_from);
            })
            .with_assertion(|world| {
                let entity_from = *world.read_resource::<Entity>();
                let frame_freeze_clocks = world.read_storage::<FrameFreezeClock>();
                let frame_freeze_clock = frame_freeze_clocks.get(entity_from);

                assert_eq!(Some(&FrameFreezeClock::new(3)), frame_freeze_clock);
            })
            .run()
    }

    #[test]
    fn multiple_hit_events_only_results_in_one_freeze_frame() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(FrameFreezeClockAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to_0 = world.create_entity().build();
                let entity_to_1 = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to_0, interaction(), body());
                send_event(world, event);
                let event = HitEvent::new(entity_from, entity_to_1, interaction(), body());
                send_event(world, event);

                world.insert(entity_from);
            })
            .with_assertion(|world| {
                let entity_from = *world.read_resource::<Entity>();
                let frame_freeze_clocks = world.read_storage::<FrameFreezeClock>();
                let frame_freeze_clock = frame_freeze_clocks.get(entity_from);

                assert_eq!(Some(&FrameFreezeClock::new(3)), frame_freeze_clock);
            })
            .run()
    }

    fn send_event(world: &mut World, event: HitEvent) {
        let mut ec = world.write_resource::<EventChannel<HitEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn interaction() -> Interaction {
        Interaction::new(InteractionKind::Hit(Hit::default()), vec![], true)
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

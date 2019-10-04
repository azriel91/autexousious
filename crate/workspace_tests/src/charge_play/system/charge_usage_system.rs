#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::{
        config::{ChargePoints, ChargeUseMode},
        play::{ChargeTrackerClock, ChargeUseEvent},
    };

    use charge_play::ChargeUsageSystem;

    #[test]
    fn charge_use_mode_defaults_to_exact() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(3));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: None,
                charge_use_event_fn,
            },
            ChargePoints::new(7),
        )
    }

    #[test]
    fn subtracts_charge_points_exact() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(3));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::Exact),
                charge_use_event_fn,
            },
            ChargePoints::new(7),
        )
    }

    #[test]
    fn subtracts_charge_points_nearest_partial_with_remainder() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(50, 28);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(25));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::NearestPartial),
                charge_use_event_fn,
            },
            ChargePoints::new(25),
        )
    }

    #[test]
    fn subtracts_charge_points_nearest_partial_without_remainder() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(100, 50);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(25));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::NearestPartial),
                charge_use_event_fn,
            },
            ChargePoints::new(25),
        )
    }

    #[test]
    fn subtracts_charge_points_nearest_whole_with_remainder() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(100, 58);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(25));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::NearestWhole),
                charge_use_event_fn,
            },
            ChargePoints::new(25),
        )
    }

    #[test]
    fn subtracts_charge_points_nearest_whole_without_remainder() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(100, 50);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(25));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::NearestWhole),
                charge_use_event_fn,
            },
            ChargePoints::new(25),
        )
    }

    #[test]
    fn subtracts_charge_points_all() -> Result<(), Error> {
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(100, 58);
        let charge_use_event_fn = |entity| ChargeUseEvent::new(entity, ChargePoints::new(25));

        run_test(
            SetupParams {
                charge_tracker_clock,
                charge_use_mode: Some(ChargeUseMode::All),
                charge_use_event_fn,
            },
            ChargePoints::new(0),
        )
    }

    macro_rules! overflow_test {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() -> Result<(), Error> {
                let charge_tracker_clock = ChargeTrackerClock::new_with_value(100, 58);
                let charge_use_event_fn =
                    |entity| ChargeUseEvent::new(entity, ChargePoints::new(100));

                run_test(
                    SetupParams {
                        charge_tracker_clock,
                        charge_use_mode: Some(ChargeUseMode::$variant),
                        charge_use_event_fn,
                    },
                    ChargePoints::new(0),
                )
            }
        };
    }

    overflow_test!(resets_charge_tracker_when_charge_use_overflows_exact, Exact);
    overflow_test!(
        resets_charge_tracker_when_charge_use_overflows_nearest_partial,
        NearestPartial
    );
    overflow_test!(
        resets_charge_tracker_when_charge_use_overflows_nearest_whole,
        NearestWhole
    );
    overflow_test!(resets_charge_tracker_when_charge_use_overflows_all, All);

    fn run_test(
        SetupParams {
            charge_tracker_clock: charge_tracker_clock_setup,
            charge_use_mode: charge_use_mode_setup,
            charge_use_event_fn,
        }: SetupParams,
        charge_points_expected: ChargePoints,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeUsageSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world.create_entity().with(charge_tracker_clock_setup);

                    if let Some(charge_use_mode_setup) = charge_use_mode_setup {
                        entity_builder = entity_builder.with(charge_use_mode_setup);
                    }

                    entity_builder.build()
                };
                let charge_use_event = charge_use_event_fn(entity);

                send_event(world, charge_use_event);

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let charge_tracker_clocks =
                    world.system_data::<ReadStorage<'_, ChargeTrackerClock>>();

                let charge_tracker_clock = charge_tracker_clocks
                    .get(entity)
                    .copied()
                    .expect("Expected `ChargeTrackerClock` component to exist.");

                assert_eq!(
                    (*charge_points_expected) as usize,
                    (*charge_tracker_clock).value
                );
            })
            .run()
    }

    fn send_event(world: &mut World, charge_use_event: ChargeUseEvent) {
        let mut charge_use_ec = world.write_resource::<EventChannel<ChargeUseEvent>>();
        charge_use_ec.single_write(charge_use_event);
    }

    struct SetupParams {
        charge_tracker_clock: ChargeTrackerClock,
        charge_use_mode: Option<ChargeUseMode>,
        charge_use_event_fn: fn(Entity) -> ChargeUseEvent,
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::{
        config::{ChargeDelay, ChargeLimit},
        play::{ChargeBeginDelayClock, ChargeDelayClock, ChargeStatus, ChargeTrackerClock},
    };

    use charge_play::ChargeInitializeDelaySystem;

    #[test]
    fn ticks_clock_when_charge_begin_delay() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new(10);
        let charge_status = ChargeStatus::BeginDelay;

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
                charge_delay: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock, charge_delay_clock| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 1);

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(None, charge_tracker_clock);
                assert_eq!(None, charge_delay_clock);
            },
        )
    }

    #[test]
    fn does_not_tick_clock_when_not_charging() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new_with_value(10, 9);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
                charge_delay: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock, charge_delay_clock| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 9);

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(None, charge_tracker_clock);
                assert_eq!(None, charge_delay_clock);
            },
        )
    }

    #[test]
    fn attaches_charge_clocks_when_begin_delay_clock_is_complete() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new_with_value(10, 9);
        let charge_status = ChargeStatus::BeginDelay;

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
                charge_delay: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock, charge_delay_clock| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 10);

                let charge_delay = ChargeDelay::default();
                let mut charge_delay_clock_expected = ChargeDelayClock::new(*charge_delay);
                (*charge_delay_clock_expected).value = *charge_delay;

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(
                    Some(ChargeTrackerClock::new((*ChargeLimit::default()) as usize)),
                    charge_tracker_clock
                );
                assert_eq!(Some(charge_delay_clock_expected), charge_delay_clock);
            },
        )
    }

    #[test]
    fn attaches_charge_tracker_clock_with_custom_limit() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new_with_value(10, 9);
        let charge_status = ChargeStatus::BeginDelay;

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: Some(ChargeLimit::new(7)),
                charge_delay: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock, _| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 10);

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(Some(ChargeTrackerClock::new(7)), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn attaches_charge_delay_clock_with_custom_delay() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new_with_value(10, 9);
        let charge_status = ChargeStatus::BeginDelay;

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
                charge_delay: Some(ChargeDelay::new(7)),
            },
            |charge_begin_delay_clock, charge_tracker_clock, charge_delay_clock| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 10);
                let charge_delay_clock_expected = ChargeDelayClock::new_with_value(7, 7);

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(
                    Some(ChargeTrackerClock::new((*ChargeLimit::default()) as usize)),
                    charge_tracker_clock
                );
                assert_eq!(Some(charge_delay_clock_expected), charge_delay_clock);
            },
        )
    }

    #[test]
    fn does_not_reset_existing_charge_tracker_clock() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new_with_value(10, 9);
        let charge_status = ChargeStatus::BeginDelay;
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(7, 4);

        run_test(
            SetupParams {
                charge_status,
                charge_begin_delay_clock,
                charge_tracker_clock: Some(charge_tracker_clock),
                charge_limit: Some(ChargeLimit::new(7)),
                charge_delay: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock, _| {
                let charge_begin_delay_clock_expected =
                    ChargeBeginDelayClock::new_with_value(10, 10);
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(7, 4);

                assert_eq!(
                    Some(charge_begin_delay_clock_expected),
                    charge_begin_delay_clock
                );
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    fn run_test(
        SetupParams {
            charge_begin_delay_clock,
            charge_status,
            charge_tracker_clock,
            charge_limit,
            charge_delay,
        }: SetupParams,
        assertion_fn: fn(
            Option<ChargeBeginDelayClock>,
            Option<ChargeTrackerClock>,
            Option<ChargeDelayClock>,
        ),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeInitializeDelaySystem::new(), "", &[])
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(charge_begin_delay_clock)
                        .with(charge_status);

                    if let Some(charge_tracker_clock) = charge_tracker_clock {
                        entity_builder = entity_builder.with(charge_tracker_clock);
                    }
                    if let Some(charge_limit) = charge_limit {
                        entity_builder = entity_builder.with(charge_limit);
                    }
                    if let Some(charge_delay) = charge_delay {
                        entity_builder = entity_builder.with(charge_delay);
                    }

                    entity_builder.build()
                };

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let (charge_begin_delay_clocks, charge_tracker_clocks, charge_delay_clocks) = world
                    .system_data::<(
                        ReadStorage<'_, ChargeBeginDelayClock>,
                        ReadStorage<'_, ChargeTrackerClock>,
                        ReadStorage<'_, ChargeDelayClock>,
                    )>();

                let charge_begin_delay_clock = charge_begin_delay_clocks.get(entity).copied();
                let charge_tracker_clock = charge_tracker_clocks.get(entity).copied();
                let charge_delay_clock = charge_delay_clocks.get(entity).copied();

                assertion_fn(
                    charge_begin_delay_clock,
                    charge_tracker_clock,
                    charge_delay_clock,
                );
            })
            .run()
    }

    struct SetupParams {
        charge_status: ChargeStatus,
        charge_begin_delay_clock: ChargeBeginDelayClock,
        charge_tracker_clock: Option<ChargeTrackerClock>,
        charge_limit: Option<ChargeLimit>,
        charge_delay: Option<ChargeDelay>,
    }
}

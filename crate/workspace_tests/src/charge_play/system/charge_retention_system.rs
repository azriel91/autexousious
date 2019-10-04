#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::{
        ChargeRetention, ChargeRetentionClock, ChargeStatus, ChargeTrackerClock,
    };

    use charge_play::ChargeRetentionSystem;

    #[test]
    fn does_nothing_for_retention_forever() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Forever;
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected = ChargeRetention::Forever;
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 10);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn resets_tracker_clock_for_retention_never_when_not_charging() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Never;
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected = ChargeRetention::Never;
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 0);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn ticks_lossy_retention_clock_when_not_charging() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Lossy(ChargeRetentionClock::new(10));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Lossy(ChargeRetentionClock::new_with_value(10, 1));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 10);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn reverse_ticks_tracker_clock_when_lossy_retention_clock_is_complete() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Lossy(ChargeRetentionClock::new_with_value(10, 9));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Lossy(ChargeRetentionClock::new_with_value(10, 0));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 9);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn ticks_reset_retention_clock_when_not_charging() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Reset(ChargeRetentionClock::new(10));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Reset(ChargeRetentionClock::new_with_value(10, 1));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 10);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn resets_tracker_clock_when_reset_retention_clock_is_complete() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Reset(ChargeRetentionClock::new_with_value(10, 9));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::NotCharging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Reset(ChargeRetentionClock::new_with_value(10, 0));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 0);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn resets_lossy_retention_clock_when_charging() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Lossy(ChargeRetentionClock::new_with_value(10, 9));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::Charging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Lossy(ChargeRetentionClock::new_with_value(10, 0));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 10);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn resets_reset_retention_clock_when_charging() -> Result<(), Error> {
        let charge_retention = ChargeRetention::Reset(ChargeRetentionClock::new_with_value(10, 9));
        let charge_tracker_clock = ChargeTrackerClock::new_with_value(10, 10);
        let charge_status = ChargeStatus::Charging;

        run_test(
            SetupParams {
                charge_status,
                charge_retention,
                charge_tracker_clock,
            },
            |charge_retention, charge_tracker_clock| {
                let charge_retention_expected =
                    ChargeRetention::Reset(ChargeRetentionClock::new_with_value(10, 0));
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 10);

                assert_eq!(Some(charge_retention_expected), charge_retention);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    fn run_test(
        SetupParams {
            charge_status,
            charge_retention,
            charge_tracker_clock,
        }: SetupParams,
        assertion_fn: fn(Option<ChargeRetention>, Option<ChargeTrackerClock>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeRetentionSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(charge_status)
                    .with(charge_retention)
                    .with(charge_tracker_clock)
                    .build();

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let (charge_retentions, charge_tracker_clocks) = world.system_data::<(
                    ReadStorage<'_, ChargeRetention>,
                    ReadStorage<'_, ChargeTrackerClock>,
                )>();

                let charge_retention = charge_retentions.get(entity).copied();
                let charge_tracker_clock = charge_tracker_clocks.get(entity).copied();

                assertion_fn(charge_retention, charge_tracker_clock);
            })
            .run()
    }

    struct SetupParams {
        charge_status: ChargeStatus,
        charge_retention: ChargeRetention,
        charge_tracker_clock: ChargeTrackerClock,
    }
}

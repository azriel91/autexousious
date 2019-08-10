use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use charge_model::play::{ChargeRetention, ChargeStatus, ChargeTrackerClock};
use derivative::Derivative;
use derive_new::new;
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Reduces charge when not charging.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeRetentionSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeRetentionSystemData<'s> {
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: ReadStorage<'s, ChargeStatus>,
    /// `ChargeRetention` components.
    #[derivative(Debug = "ignore")]
    pub charge_retentions: WriteStorage<'s, ChargeRetention>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeRetentionSystem {
    type SystemData = ChargeRetentionSystemData<'s>;

    fn run(
        &mut self,
        ChargeRetentionSystemData {
            charge_statuses,
            mut charge_retentions,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        (
            &charge_statuses,
            &mut charge_retentions,
            &mut charge_tracker_clocks,
        )
            .join()
            .for_each(|(charge_status, charge_retention, charge_tracker_clock)| {
                if *charge_status == ChargeStatus::NotCharging {
                    match charge_retention {
                        ChargeRetention::Forever => {}
                        ChargeRetention::Never => charge_tracker_clock.reset(),
                        ChargeRetention::Lossy(charge_retention_clock) => {
                            charge_retention_clock.tick();
                            if charge_retention_clock.is_complete() {
                                charge_retention_clock.reset();
                                charge_tracker_clock.reverse_tick();
                            }
                        }
                        ChargeRetention::Reset(charge_retention_clock) => {
                            charge_retention_clock.tick();
                            if charge_retention_clock.is_complete() {
                                charge_retention_clock.reset();
                                charge_tracker_clock.reset();
                            }
                        }
                    }
                } else {
                    match charge_retention {
                        ChargeRetention::Lossy(charge_retention_clock)
                        | ChargeRetention::Reset(charge_retention_clock) => {
                            charge_retention_clock.reset()
                        }
                        _ => {}
                    }
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::{
        ChargeRetention, ChargeRetentionClock, ChargeStatus, ChargeTrackerClock,
    };

    use super::ChargeRetentionSystem;

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
            .with_setup(move |world| {
                let entity = world
                    .create_entity()
                    .with(charge_status)
                    .with(charge_retention)
                    .with(charge_tracker_clock)
                    .build();

                world.add_resource(entity);
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

use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use charge_model::play::{ChargeDelayClock, ChargeStatus, ChargeTrackerClock};
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;

/// Ticks `ChargeTrackerClock` while `Charging`.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeIncrementSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeIncrementSystemData<'s> {
    /// `ChargeStatus` components.
    #[derivative(Debug = "ignore")]
    pub charge_statuses: ReadStorage<'s, ChargeStatus>,
    /// `ChargeDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_delay_clocks: WriteStorage<'s, ChargeDelayClock>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeIncrementSystem {
    type SystemData = ChargeIncrementSystemData<'s>;

    fn run(
        &mut self,
        ChargeIncrementSystemData {
            charge_statuses,
            mut charge_delay_clocks,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        (
            &charge_statuses,
            &mut charge_delay_clocks,
            &mut charge_tracker_clocks,
        )
            .join()
            .for_each(
                |(charge_status, charge_delay_clock, charge_tracker_clock)| {
                    if *charge_status == ChargeStatus::Charging {
                        charge_delay_clock.tick();

                        if charge_delay_clock.is_complete() {
                            charge_delay_clock.reset();

                            charge_tracker_clock.tick();
                        }
                    }
                },
            );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::{ChargeDelayClock, ChargeStatus, ChargeTrackerClock};

    use super::ChargeIncrementSystem;

    #[test]
    fn ticks_delay_clock_when_charging() -> Result<(), Error> {
        let charge_delay_clock = ChargeDelayClock::new(10);
        let charge_tracker_clock = ChargeTrackerClock::new(10);
        let charge_status = ChargeStatus::Charging;

        run_test(
            SetupParams {
                charge_status,
                charge_delay_clock,
                charge_tracker_clock,
            },
            |charge_delay_clock, charge_tracker_clock| {
                let charge_delay_clock_expected = ChargeDelayClock::new_with_value(10, 1);
                let charge_tracker_clock_expected = ChargeTrackerClock::new(10);

                assert_eq!(Some(charge_delay_clock_expected), charge_delay_clock);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn ticks_tracker_clock_when_delay_clock_is_complete() -> Result<(), Error> {
        let charge_delay_clock = ChargeDelayClock::new_with_value(10, 9);
        let charge_tracker_clock = ChargeTrackerClock::new(10);
        let charge_status = ChargeStatus::Charging;

        run_test(
            SetupParams {
                charge_status,
                charge_delay_clock,
                charge_tracker_clock,
            },
            |charge_delay_clock, charge_tracker_clock| {
                let charge_delay_clock_expected = ChargeDelayClock::new(10);
                let charge_tracker_clock_expected = ChargeTrackerClock::new_with_value(10, 1);

                assert_eq!(Some(charge_delay_clock_expected), charge_delay_clock);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    #[test]
    fn does_not_tick_clocks_when_not_charging() -> Result<(), Error> {
        let charge_delay_clock = ChargeDelayClock::new_with_value(10, 9);
        let charge_tracker_clock = ChargeTrackerClock::new(10);
        let charge_status = ChargeStatus::BeginDelay;

        run_test(
            SetupParams {
                charge_status,
                charge_delay_clock,
                charge_tracker_clock,
            },
            |charge_delay_clock, charge_tracker_clock| {
                let charge_delay_clock_expected = ChargeDelayClock::new_with_value(10, 9);
                let charge_tracker_clock_expected = ChargeTrackerClock::new(10);

                assert_eq!(Some(charge_delay_clock_expected), charge_delay_clock);
                assert_eq!(Some(charge_tracker_clock_expected), charge_tracker_clock);
            },
        )
    }

    fn run_test(
        SetupParams {
            charge_status,
            charge_delay_clock,
            charge_tracker_clock,
        }: SetupParams,
        assertion_fn: fn(Option<ChargeDelayClock>, Option<ChargeTrackerClock>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeIncrementSystem::new(), "", &[])
            .with_setup(move |world| {
                let entity = world
                    .create_entity()
                    .with(charge_status)
                    .with(charge_delay_clock)
                    .with(charge_tracker_clock)
                    .build();

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let (charge_delay_clocks, charge_tracker_clocks) = world.system_data::<(
                    ReadStorage<'_, ChargeDelayClock>,
                    ReadStorage<'_, ChargeTrackerClock>,
                )>();

                let charge_delay_clock = charge_delay_clocks.get(entity).copied();
                let charge_tracker_clock = charge_tracker_clocks.get(entity).copied();

                assertion_fn(charge_delay_clock, charge_tracker_clock);
            })
            .run()
    }

    struct SetupParams {
        charge_status: ChargeStatus,
        charge_delay_clock: ChargeDelayClock,
        charge_tracker_clock: ChargeTrackerClock,
    }
}

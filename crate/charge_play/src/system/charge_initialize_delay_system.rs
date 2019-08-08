use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};
use charge_model::{
    config::ChargeLimit,
    play::{ChargeBeginDelayClock, ChargeTrackerClock},
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Default limit for charge points.
const CHARGE_LIMIT_DEFAULT: ChargeLimit = ChargeLimit(100);

/// Ticks `ChargeBeginDelayClock` while `Attack` is held.
///
/// Adds `ChargeTrackerClock` on charge begin if none exists and `ChargeBeginDelayClock`
/// `is_complete()`
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeInitializeDelaySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeInitializeDelaySystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `ChargeLimit` components.
    #[derivative(Debug = "ignore")]
    pub charge_limits: ReadStorage<'s, ChargeLimit>,
    /// `ChargeBeginDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_begin_delay_clocks: WriteStorage<'s, ChargeBeginDelayClock>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeInitializeDelaySystem {
    type SystemData = ChargeInitializeDelaySystemData<'s>;

    fn run(
        &mut self,
        ChargeInitializeDelaySystemData {
            entities,
            controller_inputs,
            charge_limits,
            mut charge_begin_delay_clocks,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &controller_inputs,
            charge_limits.maybe(),
            &mut charge_begin_delay_clocks,
        )
            .join()
            .for_each(
                |(entity, controller_input, charge_limit, charge_begin_delay_clock)| {
                    if controller_input.attack {
                        charge_begin_delay_clock.tick();

                        if charge_begin_delay_clock.is_complete() {
                            if !charge_tracker_clocks.contains(entity) {
                                let charge_limit =
                                    charge_limit.copied().unwrap_or(CHARGE_LIMIT_DEFAULT);
                                let charge_limit = (*charge_limit) as usize;
                                let charge_tracker_clock = ChargeTrackerClock::new(charge_limit);
                                charge_tracker_clocks
                                    .insert(entity, charge_tracker_clock)
                                    .expect("Failed to insert `ChargeTrackerClock` component.");
                            }
                        }
                    }
                },
            );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::{
        config::ChargeLimit,
        play::{ChargeBeginDelayClock, ChargeTrackerClock},
    };
    use game_input::ControllerInput;

    use super::{ChargeInitializeDelaySystem, CHARGE_LIMIT_DEFAULT};

    #[test]
    fn ticks_clock_when_attack_is_held() -> Result<(), Error> {
        let charge_begin_delay_clock = ChargeBeginDelayClock::new(10);
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        run_test(
            SetupParams {
                controller_input,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock| {
                let mut charge_begin_delay_clock_expected = ChargeBeginDelayClock::new(10);
                (*charge_begin_delay_clock_expected).value = 1;
                assert_eq!(
                    Some(charge_begin_delay_clock_expected).as_ref(),
                    charge_begin_delay_clock
                );
                assert_eq!(None, charge_tracker_clock);
            },
        )
    }

    #[test]
    fn attaches_charge_tracker_clock_when_charge_begin_delay_clock_is_complete() -> Result<(), Error>
    {
        let mut charge_begin_delay_clock = ChargeBeginDelayClock::new(10);
        (*charge_begin_delay_clock).value = 9;
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        run_test(
            SetupParams {
                controller_input,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: None,
            },
            |charge_begin_delay_clock, charge_tracker_clock| {
                let mut charge_begin_delay_clock_expected = ChargeBeginDelayClock::new(10);
                (*charge_begin_delay_clock_expected).value = 10;
                assert_eq!(
                    Some(charge_begin_delay_clock_expected).as_ref(),
                    charge_begin_delay_clock
                );
                assert_eq!(
                    Some(ChargeTrackerClock::new((*CHARGE_LIMIT_DEFAULT) as usize)).as_ref(),
                    charge_tracker_clock
                );
            },
        )
    }

    #[test]
    fn attaches_charge_tracker_clock_with_custom_limit() -> Result<(), Error> {
        let mut charge_begin_delay_clock = ChargeBeginDelayClock::new(10);
        (*charge_begin_delay_clock).value = 9;
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        run_test(
            SetupParams {
                controller_input,
                charge_begin_delay_clock,
                charge_tracker_clock: None,
                charge_limit: Some(ChargeLimit::new(7)),
            },
            |charge_begin_delay_clock, charge_tracker_clock| {
                let mut charge_begin_delay_clock_expected = ChargeBeginDelayClock::new(10);
                (*charge_begin_delay_clock_expected).value = 10;
                assert_eq!(
                    Some(charge_begin_delay_clock_expected).as_ref(),
                    charge_begin_delay_clock
                );
                assert_eq!(
                    Some(ChargeTrackerClock::new(7)).as_ref(),
                    charge_tracker_clock
                );
            },
        )
    }

    #[test]
    fn does_not_reset_existing_charge_tracker_clock() -> Result<(), Error> {
        let mut charge_begin_delay_clock = ChargeBeginDelayClock::new(10);
        (*charge_begin_delay_clock).value = 9;
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;
        let mut charge_tracker_clock = ChargeTrackerClock::new(7);
        (*charge_tracker_clock).value = 4;

        run_test(
            SetupParams {
                controller_input,
                charge_begin_delay_clock,
                charge_tracker_clock: Some(charge_tracker_clock),
                charge_limit: Some(ChargeLimit::new(7)),
            },
            |charge_begin_delay_clock, charge_tracker_clock| {
                let mut charge_begin_delay_clock_expected = ChargeBeginDelayClock::new(10);
                (*charge_begin_delay_clock_expected).value = 10;

                let mut charge_tracker_clock_expected = ChargeTrackerClock::new(7);
                (*charge_tracker_clock_expected).value = 4;

                assert_eq!(
                    Some(charge_begin_delay_clock_expected).as_ref(),
                    charge_begin_delay_clock
                );
                assert_eq!(
                    Some(charge_tracker_clock_expected).as_ref(),
                    charge_tracker_clock
                );
            },
        )
    }

    fn run_test(
        SetupParams {
            charge_begin_delay_clock,
            controller_input,
            charge_tracker_clock,
            charge_limit,
        }: SetupParams,
        assertion_fn: fn(Option<&ChargeBeginDelayClock>, Option<&ChargeTrackerClock>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeInitializeDelaySystem::new(), "", &[])
            .with_setup(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(charge_begin_delay_clock)
                        .with(controller_input);

                    if let Some(charge_tracker_clock) = charge_tracker_clock {
                        entity_builder = entity_builder.with(charge_tracker_clock);
                    }
                    if let Some(charge_limit) = charge_limit {
                        entity_builder = entity_builder.with(charge_limit);
                    }

                    entity_builder.build()
                };

                world.add_resource(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let (charge_begin_delay_clocks, charge_tracker_clocks) = world.system_data::<(
                    ReadStorage<'_, ChargeBeginDelayClock>,
                    ReadStorage<'_, ChargeTrackerClock>,
                )>();

                let charge_begin_delay_clock = charge_begin_delay_clocks.get(entity);
                let charge_tracker_clock = charge_tracker_clocks.get(entity);

                assertion_fn(charge_begin_delay_clock, charge_tracker_clock);
            })
            .run()
    }

    struct SetupParams {
        controller_input: ControllerInput,
        charge_begin_delay_clock: ChargeBeginDelayClock,
        charge_tracker_clock: Option<ChargeTrackerClock>,
        charge_limit: Option<ChargeLimit>,
    }
}

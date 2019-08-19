use amethyst::{
    ecs::{ReadStorage, System, SystemData, World, Write, WriteStorage},
    shred::{ResourceId, Resources, SystemData},
    shrev::{EventChannel, ReaderId},
};
use charge_model::{
    config::ChargeUseMode,
    play::{ChargeTrackerClock, ChargeUseEvent},
};
use derivative::Derivative;
use derive_new::new;
use log::warn;
use typename_derive::TypeName;

/// Subtracts `ChargeTrackerClock` when used.
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeUsageSystem {
    /// Reader ID for the `ChargeUseEvent` channel.
    #[new(default)]
    charge_event_rid: Option<ReaderId<ChargeUseEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeUsageSystemData<'s> {
    /// `ChargeUseEvent` channel.
    #[derivative(Debug = "ignore")]
    pub charge_ec: Write<'s, EventChannel<ChargeUseEvent>>,
    /// `ChargeUseMode` components.
    #[derivative(Debug = "ignore")]
    pub charge_use_modes: ReadStorage<'s, ChargeUseMode>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
}

impl<'s> System<'s> for ChargeUsageSystem {
    type SystemData = ChargeUsageSystemData<'s>;

    fn run(
        &mut self,
        ChargeUsageSystemData {
            charge_ec,
            charge_use_modes,
            mut charge_tracker_clocks,
        }: Self::SystemData,
    ) {
        let charge_event_rid = self
            .charge_event_rid
            .as_mut()
            .expect("Expected `charge_event_rid` field to be set.");

        charge_ec.read(charge_event_rid).for_each(|ev| {
            let entity = ev.entity;
            let charge_use = (*ev.charge_points) as usize;

            let charge_tracker_clock = charge_tracker_clocks
                .get_mut(entity)
                .expect("Expected `ChargeTrackerClock` component to exist.");

            let charge_use_mode = charge_use_modes
                .get(entity)
                .copied()
                .unwrap_or_else(|| ChargeUseMode::Exact);

            if charge_tracker_clock.is_beginning() {
                // No charge stored.
                warn!(
                    "Attempted to subtract `{}` charge points from empty charge.",
                    charge_use,
                );

                return;
            }

            let charge_current = (*charge_tracker_clock).value;
            if charge_use > charge_current {
                if charge_use_mode != ChargeUseMode::NearestPartial {
                    warn!(
                        "Attempted to subtract `{}` charge points when `{}` available.",
                        charge_use, charge_current
                    );
                }

                charge_tracker_clock.reset();
                return;
            }

            match charge_use_mode {
                ChargeUseMode::Exact => (*charge_tracker_clock).value = charge_current - charge_use,
                ChargeUseMode::NearestPartial => {
                    let remainder = charge_current % charge_use;
                    if remainder > 0 {
                        (*charge_tracker_clock).value -= remainder;
                    } else {
                        (*charge_tracker_clock).value -= charge_use;
                    }
                }
                ChargeUseMode::NearestWhole => {
                    let remainder = charge_current % charge_use;
                    (*charge_tracker_clock).value -= charge_use + remainder;
                }
                ChargeUseMode::All => charge_tracker_clock.reset(),
            }
        });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.charge_event_rid = Some(
            res.fetch_mut::<EventChannel<ChargeUseEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, World},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::{
        config::{ChargePoints, ChargeUseMode},
        play::{ChargeTrackerClock, ChargeUseEvent},
    };

    use super::ChargeUsageSystem;

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
            .with_setup(move |world| {
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

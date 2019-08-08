use amethyst::{
    ecs::{Read, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use charge_model::play::ChargeBeginDelayClock;
use derivative::Derivative;
use derive_new::new;
use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Default number of ticks to wait before beginning to charge.
const CHARGE_DELAY_DEFAULT: usize = 10;

/// Detects the begin / cancellation of the initialization phase of charging.
///
/// Resets `ChargeBeginDelayClock` upon charge start / stop (currently control input event release).
#[derive(Debug, Default, TypeName, new)]
pub struct ChargeInitializeDetectionSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChargeInitializeDetectionSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `ChargeBeginDelayClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_begin_delay_clocks: WriteStorage<'s, ChargeBeginDelayClock>,
}

impl<'s> System<'s> for ChargeInitializeDetectionSystem {
    type SystemData = ChargeInitializeDetectionSystemData<'s>;

    fn run(
        &mut self,
        ChargeInitializeDetectionSystemData {
            control_input_ec,
            mut charge_begin_delay_clocks,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .filter_map(|ev| match ev {
                ControlInputEvent::ControlActionPress(ControlActionEventData {
                    entity,
                    control_action,
                })
                | ControlInputEvent::ControlActionRelease(ControlActionEventData {
                    entity,
                    control_action,
                }) => {
                    if *control_action == ControlAction::Attack {
                        Some(*entity)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .for_each(|entity| {
                if let Some(charge_begin_delay_clock) = charge_begin_delay_clocks.get_mut(entity) {
                    charge_begin_delay_clock.reset();
                } else {
                    charge_begin_delay_clocks
                        .insert(entity, ChargeBeginDelayClock::new(CHARGE_DELAY_DEFAULT))
                        .expect("Failed to insert `ChargeBeginDelayClock` component.");
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.control_input_event_rid = Some(
            res.fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::ChargeBeginDelayClock;
    use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};

    use super::{ChargeInitializeDetectionSystem, CHARGE_DELAY_DEFAULT};

    #[test]
    fn inserts_charge_begin_delay_clock_on_attack_press() -> Result<(), Error> {
        run_test(None, press_attack, |charge_begin_delay_clock| {
            assert_eq!(
                Some(ChargeBeginDelayClock::new(CHARGE_DELAY_DEFAULT)).as_ref(),
                charge_begin_delay_clock
            );
        })
    }

    #[test]
    fn resets_charge_begin_delay_clock_on_attack_release() -> Result<(), Error> {
        let mut charge_begin_delay_clock = ChargeBeginDelayClock::new(5);
        (*charge_begin_delay_clock).value = 3;

        run_test(
            Some(charge_begin_delay_clock),
            release_attack,
            |charge_begin_delay_clock| {
                assert_eq!(
                    Some(ChargeBeginDelayClock::new(5)).as_ref(),
                    charge_begin_delay_clock
                );
            },
        )
    }

    #[test]
    fn does_not_insert_charge_begin_delay_clock_on_non_attack_press() -> Result<(), Error> {
        run_test(None, press_jump, |charge_begin_delay_clock| {
            assert_eq!(None, charge_begin_delay_clock);
        })
    }

    fn run_test(
        charge_begin_delay_clock: Option<ChargeBeginDelayClock>,
        control_input_event_fn: fn(Entity) -> ControlInputEvent,
        assertion_fn: fn(Option<&ChargeBeginDelayClock>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeInitializeDetectionSystem::new(), "", &[])
            .with_setup(move |world| {
                let entity = {
                    let mut entity_builder = world.create_entity();

                    if let Some(charge_begin_delay_clock) = charge_begin_delay_clock {
                        entity_builder = entity_builder.with(charge_begin_delay_clock);
                    }

                    entity_builder.build()
                };
                let control_input_event = control_input_event_fn(entity);

                world
                    .write_resource::<EventChannel<ControlInputEvent>>()
                    .single_write(control_input_event);

                world.add_resource(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let charge_begin_delay_clocks = world.read_storage::<ChargeBeginDelayClock>();

                let charge_begin_delay_clock = charge_begin_delay_clocks.get(entity);
                assertion_fn(charge_begin_delay_clock);
            })
            .run()
    }

    fn press_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Attack,
        })
    }

    fn press_jump(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Jump,
        })
    }

    fn release_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionRelease(ControlActionEventData {
            entity,
            control_action: ControlAction::Attack,
        })
    }
}

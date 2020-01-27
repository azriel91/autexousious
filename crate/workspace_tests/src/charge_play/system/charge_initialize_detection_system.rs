#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, ReadStorage, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::{ChargeBeginDelayClock, ChargeStatus};
    use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};

    use charge_play::{ChargeInitializeDetectionSystem, CHARGE_DELAY_DEFAULT};

    #[test]
    fn inserts_charge_begin_delay_clock_and_sets_charging_status_on_attack_press(
    ) -> Result<(), Error> {
        run_test(
            None,
            press_attack,
            |charge_begin_delay_clock, charge_status| {
                assert_eq!(
                    Some(ChargeBeginDelayClock::new(CHARGE_DELAY_DEFAULT)),
                    charge_begin_delay_clock
                );

                assert_eq!(Some(ChargeStatus::BeginDelay), charge_status);
            },
        )
    }

    #[test]
    fn resets_charge_begin_delay_clock_on_attack_release() -> Result<(), Error> {
        let charge_begin_delay_clock =
            ChargeBeginDelayClock::new_with_value(CHARGE_DELAY_DEFAULT, 3);

        run_test(
            Some(charge_begin_delay_clock),
            release_attack,
            |charge_begin_delay_clock, charge_status| {
                assert_eq!(
                    Some(ChargeBeginDelayClock::new(CHARGE_DELAY_DEFAULT)),
                    charge_begin_delay_clock
                );

                assert_eq!(Some(ChargeStatus::NotCharging), charge_status);
            },
        )
    }

    #[test]
    fn does_not_insert_charge_begin_delay_clock_on_non_attack_press() -> Result<(), Error> {
        run_test(
            None,
            press_jump,
            |charge_begin_delay_clock, charge_status| {
                assert_eq!(None, charge_begin_delay_clock);

                assert_eq!(None, charge_status);
            },
        )
    }

    fn run_test(
        charge_begin_delay_clock: Option<ChargeBeginDelayClock>,
        control_input_event_fn: fn(Entity) -> ControlInputEvent,
        assertion_fn: fn(Option<ChargeBeginDelayClock>, Option<ChargeStatus>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(ChargeInitializeDetectionSystem::new(), "", &[])
            .with_effect(move |world| {
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

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let (charge_begin_delay_clocks, charge_statuses) = world.system_data::<(
                    ReadStorage<'_, ChargeBeginDelayClock>,
                    ReadStorage<'_, ChargeStatus>,
                )>();

                let charge_begin_delay_clock = charge_begin_delay_clocks.get(entity).copied();
                let charge_status = charge_statuses.get(entity).copied();
                assertion_fn(charge_begin_delay_clock, charge_status);
            })
            .run()
    }

    fn press_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            controller_id: 0,
            entity,
            control_action: ControlAction::Attack,
        })
    }

    fn press_jump(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            controller_id: 0,
            entity,
            control_action: ControlAction::Jump,
        })
    }

    fn release_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionRelease(ControlActionEventData {
            controller_id: 0,
            entity,
            control_action: ControlAction::Attack,
        })
    }
}

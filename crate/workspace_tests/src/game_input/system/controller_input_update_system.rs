#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlBindings,
        ControlInputEvent,
    };

    use game_input::{ControllerInput, ControllerInputUpdateSystem};

    #[test]
    fn inserts_controller_input_if_non_existent() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                ControllerInputUpdateSystem::new(),
                any::type_name::<ControllerInputUpdateSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(|world| {
                let e0 = world.create_entity().build();
                let e1 = world.create_entity().build();

                // Write events.
                world
                    .write_resource::<EventChannel<ControlInputEvent>>()
                    .drain_vec_write(&mut vec![
                        ControlInputEvent::AxisMoved(AxisMoveEventData {
                            entity: e0.clone(),
                            axis: Axis::X,
                            value: 1.,
                        }),
                        ControlInputEvent::AxisMoved(AxisMoveEventData {
                            entity: e0.clone(),
                            axis: Axis::Z,
                            value: 1.,
                        }),
                        ControlInputEvent::ControlActionPress(ControlActionEventData {
                            entity: e1.clone(),
                            control_action: ControlAction::Defend,
                        }),
                    ]); // kcov-ignore

                world.insert((e0, e1));
            })
            .with_assertion(|world| {
                let entities = world.read_resource::<(Entity, Entity)>();
                let e0 = &entities.0;
                let e1 = &entities.1;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(1., 1., false, false, false, false)),
                    store.get(*e0)
                );
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., true, false, false, false)),
                    store.get(*e1)
                );
            })
            .run()
    }

    #[test]
    fn updates_controller_input_from_control_input_events() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                ControllerInputUpdateSystem::new(),
                any::type_name::<ControllerInputUpdateSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(|world| {
                let e0 = world
                    .create_entity()
                    .with(ControllerInput::new(1., -1., true, true, false, false))
                    .build();

                let e1 = world
                    .create_entity()
                    .with(ControllerInput::new(1., -1., true, true, false, false))
                    .build();

                // Write events.
                world
                    .write_resource::<EventChannel<ControlInputEvent>>()
                    .drain_vec_write(&mut vec![
                        ControlInputEvent::AxisMoved(AxisMoveEventData {
                            entity: e0.clone(),
                            axis: Axis::X,
                            value: 0.,
                        }),
                        ControlInputEvent::AxisMoved(AxisMoveEventData {
                            entity: e0.clone(),
                            axis: Axis::Z,
                            value: 1.,
                        }),
                        // e1
                        ControlInputEvent::ControlActionRelease(ControlActionEventData {
                            entity: e1.clone(),
                            control_action: ControlAction::Defend,
                        }),
                        ControlInputEvent::ControlActionRelease(ControlActionEventData {
                            entity: e1.clone(),
                            control_action: ControlAction::Jump,
                        }),
                        ControlInputEvent::ControlActionPress(ControlActionEventData {
                            entity: e1.clone(),
                            control_action: ControlAction::Attack,
                        }),
                        ControlInputEvent::ControlActionPress(ControlActionEventData {
                            entity: e1.clone(),
                            control_action: ControlAction::Special,
                        }),
                    ]); // kcov-ignore

                world.insert((e0, e1));
            })
            .with_assertion(|world| {
                let entities = world.read_resource::<(Entity, Entity)>();
                let e0 = &entities.0;
                let e1 = &entities.1;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 1., true, true, false, false)),
                    store.get(*e0)
                );
                assert_eq!(
                    Some(&ControllerInput::new(1., -1., false, false, true, true)),
                    store.get(*e1)
                );
            })
            .run()
    }
}

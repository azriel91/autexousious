#[cfg(test)]
mod test {
    use std::{collections::HashMap, convert::TryFrom};

    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        input::{Axis as InputAxis, Bindings, Button, InputEvent, InputHandler},
        shrev::{EventChannel, ReaderId},
        winit::{
            DeviceId, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode,
            WindowEvent, WindowId,
        },
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI};
    use game_input::{InputControlled, SharedInputControlled};
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlBindings,
        ControlInputEvent, ControllerConfig, InputConfig,
    };
    use hamcrest::prelude::*;
    use indexmap::IndexMap;
    use typename::TypeName;

    use game_input_ui::InputToControlInputSystem;

    const ACTION_JUMP: VirtualKeyCode = VirtualKeyCode::Key1;
    const AXIS_POSITIVE: VirtualKeyCode = VirtualKeyCode::D;
    const AXIS_NEGATIVE: VirtualKeyCode = VirtualKeyCode::A;

    #[test]
    fn sends_control_input_events_for_key_presses() -> Result<(), Error> {
        run_test(
            vec![key_press(AXIS_POSITIVE), key_press(ACTION_JUMP)],
            |input_controlled_entity, shared_input_controlled_entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity: shared_input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                ]
            },
        )
    }

    #[test]
    fn sends_control_input_events_for_key_releases() -> Result<(), Error> {
        run_test(
            vec![
                key_press(AXIS_POSITIVE),
                key_release(AXIS_POSITIVE),
                key_press(ACTION_JUMP),
                key_release(ACTION_JUMP),
            ],
            |input_controlled_entity, shared_input_controlled_entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity: shared_input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        entity: shared_input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                ]
            },
        )
    }

    fn run_test<F>(key_events: Vec<Event>, expected_control_input_events: F) -> Result<(), Error>
    where
        F: Send + Sync + Fn(Entity, Entity) -> Vec<ControlInputEvent> + 'static,
    {
        let input_config = input_config();
        let bindings = Bindings::<ControlBindings>::try_from(&input_config)?;

        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                InputToControlInputSystem::new(input_config),
                InputToControlInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                // HACK: This is what `InputSystem` does from `amethyst::input::InputBundle` in the
                // system setup phase.
                // TODO: Update `amethyst_test` to take in `InputBindings`.
                world
                    .write_resource::<InputHandler<ControlBindings>>()
                    .bindings = bindings.clone();

                let reader_id = world
                    .write_resource::<EventChannel<ControlInputEvent>>()
                    .register_reader(); // kcov-ignore
                world.insert(reader_id);

                let controller_id = 0;
                let input_controlled_entity = world
                    .create_entity()
                    .with(InputControlled::new(controller_id))
                    .build();
                let shared_input_controlled_entity =
                    world.create_entity().with(SharedInputControlled).build();
                world.insert((input_controlled_entity, shared_input_controlled_entity));

                // Use the same closure so that the system does not send events before we send the
                // key events.

                let mut input_handler = world.write_resource::<InputHandler<ControlBindings>>();
                let mut input_events_ec =
                    world.write_resource::<EventChannel<InputEvent<ControlBindings>>>();

                key_events.iter().for_each(|ev| {
                    input_handler.send_event(ev, &mut input_events_ec, HIDPI as f32)
                });
            })
            .with_assertion(move |world| {
                let input_events = {
                    let input_events_ec = world.read_resource::<EventChannel<ControlInputEvent>>();
                    let mut input_events_id = world.write_resource::<ReaderId<ControlInputEvent>>();
                    input_events_ec
                        .read(&mut input_events_id)
                        .copied()
                        .collect::<Vec<ControlInputEvent>>()
                };

                let (input_controlled_entity, shared_input_controlled_entity) =
                    *world.read_resource::<(Entity, Entity)>();
                assert_that!(
                    &input_events,
                    contains(expected_control_input_events(
                        input_controlled_entity,
                        shared_input_controlled_entity
                    ))
                    .exactly()
                    .in_order()
                );
            })
            .run()
    }

    fn input_config() -> InputConfig {
        let controller_config_0 = controller_config([AXIS_NEGATIVE, AXIS_POSITIVE, ACTION_JUMP]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let mut controller_configs = IndexMap::new();
        controller_configs.insert(String::from("zero1"), controller_config_0);
        controller_configs.insert(String::from("one"), controller_config_1);
        InputConfig::new(controller_configs)
    }

    fn controller_config(keys: [VirtualKeyCode; 3]) -> ControllerConfig {
        let mut axes = HashMap::new();
        axes.insert(
            Axis::X,
            InputAxis::Emulated {
                neg: Button::Key(keys[0]),
                pos: Button::Key(keys[1]),
            },
        );
        let mut actions = HashMap::new();
        actions.insert(ControlAction::Jump, Button::Key(keys[2]));
        ControllerConfig::new(axes, actions)
    }

    fn key_press(virtual_keycode: VirtualKeyCode) -> Event {
        key_event(virtual_keycode, ElementState::Pressed)
    }

    fn key_release(virtual_keycode: VirtualKeyCode) -> Event {
        key_event(virtual_keycode, ElementState::Released)
    }

    fn key_event(virtual_keycode: VirtualKeyCode, state: ElementState) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode: 404,
                    state,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        logo: false,
                    },
                },
            },
        }
    }
}

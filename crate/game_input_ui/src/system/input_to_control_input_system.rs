use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, Resources, System, SystemData, World, Write},
    input::InputEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::{
    AxisMoveEventData, ControlActionEventData, ControlBindings, ControlInputEvent, InputConfig,
    PlayerActionControl, PlayerAxisControl,
};
use typename_derive::TypeName;

/// Sends `ControlInputEvent`s based on the `InputHandler` state.
#[derive(Debug, Default, TypeName, new)]
pub struct InputToControlInputSystem {
    /// All controller input configuration.
    input_config: InputConfig,
    /// Reader ID for the `InputEvent` channel.
    #[new(default)]
    input_event_rid: Option<ReaderId<InputEvent<ControlBindings>>>,
    /// Pre-allocated vector
    #[new(value = "Vec::with_capacity(64)")]
    control_input_events: Vec<ControlInputEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputToControlInputSystemData<'s> {
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Read<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Write<'s, EventChannel<ControlInputEvent>>,
}

impl<'s> System<'s> for InputToControlInputSystem {
    type SystemData = InputToControlInputSystemData<'s>;

    fn run(
        &mut self,
        InputToControlInputSystemData {
            input_ec,
            entities,
            input_controlleds,
            mut control_input_ec,
        }: Self::SystemData,
    ) {
        let input_event_rid = self
            .input_event_rid
            .as_mut()
            .expect("Expected `input_event_rid` field to be set.");

        input_ec.read(input_event_rid).for_each(|ev| {
            let control_input_event = match ev {
                InputEvent::ActionPressed(PlayerActionControl { player, action }) => {
                    // Find the entity has the `player` control id in its `InputControlled`
                    // component.

                    if let Some((entity, _)) = (&entities, &input_controlleds).join().find(
                        |(_entity, input_controlled)| input_controlled.controller_id == *player,
                    ) {
                        Some(ControlInputEvent::ControlActionPress(
                            ControlActionEventData {
                                entity,
                                control_action: *action,
                            },
                        ))
                    } else {
                        None
                    }
                }
                InputEvent::ActionReleased(PlayerActionControl { player, action }) => {
                    if let Some((entity, _)) = (&entities, &input_controlleds).join().find(
                        |(_entity, input_controlled)| input_controlled.controller_id == *player,
                    ) {
                        Some(ControlInputEvent::ControlActionRelease(
                            ControlActionEventData {
                                entity,
                                control_action: *action,
                            },
                        ))
                    } else {
                        None
                    }
                }
                InputEvent::AxisMoved {
                    axis: PlayerAxisControl { player, axis },
                    value,
                } => {
                    if let Some((entity, _)) = (&entities, &input_controlleds).join().find(
                        |(_entity, input_controlled)| input_controlled.controller_id == *player,
                    ) {
                        Some(ControlInputEvent::AxisMoved(AxisMoveEventData {
                            entity,
                            axis: *axis,
                            value: *value,
                        }))
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(control_input_event) = control_input_event {
                self.control_input_events.push(control_input_event);
            }
        });

        control_input_ec.drain_vec_write(&mut self.control_input_events);
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        // TODO: figure out how to implement controller configuration updates, because we need to
        // update the resource and what this system stores.
        res.insert(self.input_config.clone());

        self.input_event_rid = Some(
            res.fetch_mut::<EventChannel<InputEvent<ControlBindings>>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, convert::TryFrom};

    use amethyst::{
        ecs::{Builder, Entity},
        input::{Axis as InputAxis, Bindings, Button, InputEvent, InputHandler},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI};
    use game_input::InputControlled;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlBindings,
        ControlInputEvent, ControllerConfig, InputConfig,
    };
    use hamcrest::prelude::*;
    use typename::TypeName;
    use winit::{
        DeviceId, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent,
        WindowId,
    };

    use super::InputToControlInputSystem;

    const ACTION_JUMP: VirtualKeyCode = VirtualKeyCode::Key1;
    const AXIS_POSITIVE: VirtualKeyCode = VirtualKeyCode::D;
    const AXIS_NEGATIVE: VirtualKeyCode = VirtualKeyCode::A;

    #[test]
    fn sends_control_input_events_for_key_presses() -> Result<(), Error> {
        run_test(
            vec![key_press(AXIS_POSITIVE), key_press(ACTION_JUMP)],
            |entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity,
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
            |entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        entity,
                        control_action: ControlAction::Jump,
                    }),
                ]
            },
        )
    }

    fn run_test<F>(key_events: Vec<Event>, expected_control_input_events: F) -> Result<(), Error>
    where
        F: Send + Sync + Fn(Entity) -> Vec<ControlInputEvent> + 'static,
    {
        let input_config = input_config();
        let bindings = Bindings::<ControlBindings>::try_from(&input_config)?;

        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                InputToControlInputSystem::new(input_config),
                InputToControlInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(move |world| {
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
                let entity = world
                    .create_entity()
                    .with(InputControlled::new(controller_id))
                    .build();
                world.insert(entity);

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
                        .map(|ev| *ev)
                        .collect::<Vec<ControlInputEvent>>()
                };
                let entity = world.read_resource::<Entity>().clone();

                assert_that!(
                    &input_events,
                    contains(expected_control_input_events(entity))
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

        let controller_configs = vec![controller_config_0, controller_config_1];
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

use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, Resources, System, SystemData, Write},
    input::InputHandler,
    shrev::EventChannel,
};
use derive_new::new;
use game_input_model::{
    Axis, AxisEventData, ControlAction, ControlActionEventData, ControlInputEvent, InputConfig,
    PlayerActionControl, PlayerAxisControl,
};
use strum::IntoEnumIterator;
use typename_derive::TypeName;

use crate::{ControllerInput, InputControlled};

/// Sends `ControlInputEvent`s based on the `InputHandler` state.
#[derive(Debug, Default, TypeName, new)]
pub struct InputToControlInputSystem {
    /// All controller input configuration.
    input_config: InputConfig,
    /// Pre-allocated vector
    #[new(value = "Vec::with_capacity(64)")]
    input_events: Vec<ControlInputEvent>,
}

type InputToControlInputSystemData<'s> = (
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    Entities<'s>,
    ReadStorage<'s, InputControlled>,
    ReadStorage<'s, ControllerInput>,
    Write<'s, EventChannel<ControlInputEvent>>,
);

impl<'s> System<'s> for InputToControlInputSystem {
    type SystemData = InputToControlInputSystemData<'s>;

    fn run(
        &mut self,
        (input_handler, entities, input_controlleds, controller_inputs, mut input_events_ec): Self::SystemData,
    ) {
        // This does not send events when there is no existing `ControllerInput` component attached
        // to the entity. This is to prevent events from being sent when we are restoring state,
        // e.g. in a saveload scenario.
        for (entity, input_controlled, controller_input) in
            (&*entities, &input_controlleds, &controller_inputs).join()
        {
            let controller_id = input_controlled.controller_id;

            Axis::iter().for_each(|axis| {
                if let Some(value) =
                    input_handler.axis_value(&PlayerAxisControl::new(controller_id, axis))
                {
                    let previous_value = match axis {
                        Axis::X => controller_input.x_axis_value,
                        Axis::Z => controller_input.z_axis_value,
                    };

                    if previous_value != value {
                        self.input_events
                            .push(ControlInputEvent::Axis(AxisEventData {
                                entity: entity.clone(),
                                axis,
                                value,
                            }))
                    }
                }
            });

            ControlAction::iter().for_each(|control_action| {
                if let Some(value) = input_handler
                    .action_is_down(&PlayerActionControl::new(controller_id, control_action))
                {
                    let previous_value = match control_action {
                        ControlAction::Defend => controller_input.defend,
                        ControlAction::Jump => controller_input.jump,
                        ControlAction::Attack => controller_input.attack,
                        ControlAction::Special => controller_input.special,
                    };

                    if previous_value != value {
                        self.input_events.push(ControlInputEvent::ControlAction(
                            ControlActionEventData {
                                entity: entity.clone(),
                                control_action,
                                value,
                            },
                        ))
                    }
                }
            });
        }

        input_events_ec.drain_vec_write(&mut self.input_events);
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        // TODO: figure out how to implement controller configuration updates, because we need to
        // update the resource and what this system stores.
        res.insert(self.input_config.clone());
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use amethyst::{
        ecs::{Builder, Entity},
        input::{Axis as InputAxis, Bindings, Button, InputEvent, InputHandler},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI};
    use game_input_model::{
        Axis, AxisEventData, ControlAction, ControlActionEventData, ControlInputEvent,
        ControllerConfig, InputConfig, PlayerActionControl, PlayerAxisControl,
    };
    use hamcrest::prelude::*;
    use typename::TypeName;
    use winit::{
        DeviceId, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent,
        WindowId,
    };

    use super::InputToControlInputSystem;
    use crate::{ControllerInput, InputControlled};

    #[test]
    fn sends_control_input_events_for_key_presses() -> Result<(), Error> {
        run_test(
            ControllerInput::default(),
            vec![
                key_press(VirtualKeyCode::D),
                key_press(VirtualKeyCode::Key1),
            ],
            |entity| {
                vec![
                    ControlInputEvent::Axis(AxisEventData {
                        entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::ControlAction(ControlActionEventData {
                        entity,
                        control_action: ControlAction::Jump,
                        value: true,
                    }),
                ]
            },
        )
    }

    #[test]
    fn sends_control_input_events_for_key_releases() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.jump = true;

        run_test(
            controller_input,
            vec![
                key_release(VirtualKeyCode::D),
                key_release(VirtualKeyCode::Key1),
            ],
            |entity| {
                vec![
                    ControlInputEvent::Axis(AxisEventData {
                        entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::ControlAction(ControlActionEventData {
                        entity,
                        control_action: ControlAction::Jump,
                        value: false,
                    }),
                ]
            },
        )
    }

    #[test]
    fn does_not_send_control_input_events_for_non_state_change() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.jump = true;

        run_test(
            controller_input,
            vec![
                key_press(VirtualKeyCode::D),
                key_press(VirtualKeyCode::Key1),
            ],
            |_entity| vec![],
        )
    }

    fn run_test<F>(
        controller_input: ControllerInput,
        key_events: Vec<Event>,
        expected_control_input_events: F,
    ) -> Result<(), Error>
    where
        F: Send + Sync + Fn(Entity) -> Vec<ControlInputEvent> + 'static,
    {
        let input_config = input_config();
        let bindings = Bindings::<PlayerAxisControl, PlayerActionControl>::from(&input_config);

        AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
            .with_system(
                InputToControlInputSystem::new(input_config),
                InputToControlInputSystem::type_name(),
                &[],
            )
            .with_setup(move |world| {
                // HACK: This is what `InputSystem` does from `amethyst::input::InputBundle` in the
                // system setup phase.
                // TODO: Update `amethyst_test` to take in `InputBindings`.
                world
                    .write_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>()
                    .bindings = bindings.clone();

                let reader_id = world
                    .write_resource::<EventChannel<ControlInputEvent>>()
                    .register_reader();
                world.add_resource(reader_id);

                let controller_id = 0;
                let entity = world
                    .create_entity()
                    .with(InputControlled::new(controller_id))
                    .with(controller_input)
                    .build();
                world.add_resource(entity);

                // Use the same closure so that the system does not send events before we send the
                // key events.

                let mut input_handler =
                    world.write_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>();
                let mut input_events_ec =
                    world.write_resource::<EventChannel<InputEvent<PlayerActionControl>>>();

                key_events
                    .iter()
                    .for_each(|ev| input_handler.send_event(ev, &mut input_events_ec, HIDPI));
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
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
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

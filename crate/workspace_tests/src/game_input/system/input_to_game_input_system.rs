#[cfg(test)]
mod tests {
    use std::{any, collections::HashMap, convert::TryFrom};

    use amethyst::{
        ecs::WorldExt,
        input::{Axis as InputAxis, Bindings, Button, InputEvent, InputHandler},
        shrev::{EventChannel, ReaderId},
        winit::{
            event::{
                DeviceId, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode,
                WindowEvent,
            },
            window::WindowId,
        },
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI};
    use game_input_model::{
        config::{
            Axis, ControlAction, ControlBindings, ControllerConfig, PlayerActionControl,
            PlayerAxisControl, PlayerInputConfig, PlayerInputConfigs,
        },
        GameInputEvent,
    };
    use hamcrest::prelude::*;

    use game_input::{InputToGameInputSystem, InputToGameInputSystemDesc};

    const ACTION_JUMP: VirtualKeyCode = VirtualKeyCode::Key1;
    const AXIS_POSITIVE: VirtualKeyCode = VirtualKeyCode::D;
    const AXIS_NEGATIVE: VirtualKeyCode = VirtualKeyCode::A;

    #[test]
    fn sends_input_events_for_key_presses() -> Result<(), Error> {
        run_test(
            vec![key_press(AXIS_POSITIVE), key_press(ACTION_JUMP)],
            vec![
                GameInputEvent::AxisMoved {
                    axis: PlayerAxisControl {
                        player: 0,
                        axis: Axis::X,
                    },
                    value: 1.,
                },
                GameInputEvent::ActionPressed(PlayerActionControl {
                    player: 0,
                    action: ControlAction::Jump,
                }),
            ],
        )
    }

    #[test]
    fn sends_input_events_for_key_releases() -> Result<(), Error> {
        run_test(
            vec![
                key_press(AXIS_POSITIVE),
                key_release(AXIS_POSITIVE),
                key_press(ACTION_JUMP),
                key_release(ACTION_JUMP),
            ],
            vec![
                GameInputEvent::AxisMoved {
                    axis: PlayerAxisControl {
                        player: 0,
                        axis: Axis::X,
                    },
                    value: 1.,
                },
                GameInputEvent::AxisMoved {
                    axis: PlayerAxisControl {
                        player: 0,
                        axis: Axis::X,
                    },
                    value: 0.,
                },
                GameInputEvent::ActionPressed(PlayerActionControl {
                    player: 0,
                    action: ControlAction::Jump,
                }),
                GameInputEvent::ActionReleased(PlayerActionControl {
                    player: 0,
                    action: ControlAction::Jump,
                }),
            ],
        )
    }

    fn run_test(
        key_events: Vec<Event<'static, ()>>,
        input_events_expected: Vec<GameInputEvent>,
    ) -> Result<(), Error> {
        let player_input_configs = player_input_configs();
        let bindings = Bindings::<ControlBindings>::try_from(&player_input_configs)?;

        AmethystApplication::ui_base::<ControlBindings>()
            .with_resource(player_input_configs)
            .with_system_desc(
                InputToGameInputSystemDesc::default(),
                any::type_name::<InputToGameInputSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                // HACK: This is what `InputSystem` does from `amethyst::input::InputBundle` in
                // the system setup phase.
                // TODO: Update `amethyst_test` to take in `InputBindings`.
                world
                    .write_resource::<InputHandler<ControlBindings>>()
                    .bindings = bindings.clone();

                let reader_id = world
                    .write_resource::<EventChannel<GameInputEvent>>()
                    .register_reader(); // kcov-ignore
                world.insert(reader_id);

                // Use the same closure so that the system does not send events before we send
                // the key events.

                let mut input_handler = world.write_resource::<InputHandler<ControlBindings>>();
                let mut input_ec =
                    world.write_resource::<EventChannel<InputEvent<ControlBindings>>>();

                key_events
                    .iter()
                    .for_each(|ev| input_handler.send_event(ev, &mut input_ec, HIDPI as f32));
            })
            .with_assertion(move |world| {
                let input_events = {
                    let game_input_ec = world.read_resource::<EventChannel<GameInputEvent>>();
                    let mut game_input_event_rid =
                        world.write_resource::<ReaderId<GameInputEvent>>();
                    game_input_ec
                        .read(&mut game_input_event_rid)
                        .copied()
                        .collect::<Vec<GameInputEvent>>()
                };

                assert_that!(
                    &input_events,
                    contains(input_events_expected).exactly().in_order()
                );
            })
            .run()
    }

    fn player_input_configs() -> PlayerInputConfigs {
        let controller_config_0 = controller_config([AXIS_NEGATIVE, AXIS_POSITIVE, ACTION_JUMP]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let player_input_config_0 =
            PlayerInputConfig::new(String::from("zero1"), controller_config_0);
        let player_input_config_1 =
            PlayerInputConfig::new(String::from("one"), controller_config_1);

        PlayerInputConfigs::new(vec![player_input_config_0, player_input_config_1])
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

    fn key_press(virtual_keycode: VirtualKeyCode) -> Event<'static, ()> {
        key_event(virtual_keycode, ElementState::Pressed)
    }

    fn key_release(virtual_keycode: VirtualKeyCode) -> Event<'static, ()> {
        key_event(virtual_keycode, ElementState::Released)
    }

    #[allow(deprecated)]
    fn key_event(virtual_keycode: VirtualKeyCode, state: ElementState) -> Event<'static, ()> {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                is_synthetic: true,
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode: 404,
                    state,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState::default(),
                },
            },
        }
    }
}

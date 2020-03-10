#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::{
        config::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl},
        play::{
            AxisMoveEventData, ControlActionEventData, ControlInputEvent, InputControlled,
            SharedInputControlled,
        },
        GameInputEvent,
    };
    use hamcrest::prelude::*;

    use game_input::{GameInputToControlInputSystem, GameInputToControlInputSystemDesc};

    #[test]
    fn sends_control_input_events_for_key_presses() -> Result<(), Error> {
        run_test(
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
            |input_controlled_entity, shared_input_controlled_entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id: 0,
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
            |input_controlled_entity, shared_input_controlled_entity| {
                vec![
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 1.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::AxisMoved(AxisMoveEventData {
                        controller_id: 0,
                        entity: shared_input_controlled_entity,
                        axis: Axis::X,
                        value: 0.,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionPress(ControlActionEventData {
                        controller_id: 0,
                        entity: shared_input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        controller_id: 0,
                        entity: input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                    ControlInputEvent::ControlActionRelease(ControlActionEventData {
                        controller_id: 0,
                        entity: shared_input_controlled_entity,
                        control_action: ControlAction::Jump,
                    }),
                ]
            },
        )
    }

    fn run_test<F>(
        mut game_input_events: Vec<GameInputEvent>,
        expected_control_input_events: F,
    ) -> Result<(), Error>
    where
        F: Send + Sync + Fn(Entity, Entity) -> Vec<ControlInputEvent> + 'static,
    {
        AmethystApplication::blank()
            .with_system_desc(
                GameInputToControlInputSystemDesc::default(),
                any::type_name::<GameInputToControlInputSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
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

                let mut game_input_ec = world.write_resource::<EventChannel<GameInputEvent>>();

                game_input_ec.drain_vec_write(&mut game_input_events);
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
}

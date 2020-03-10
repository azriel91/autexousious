use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::{PlayerActionControl, PlayerAxisControl},
    play::{
        AxisMoveEventData, ControlActionEventData, ControlInputEvent, InputControlled,
        SharedInputControlled,
    },
    GameInputEvent,
};

/// Sends `ControlInputEvent`s based on the `InputHandler` state.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(GameInputToControlInputSystemDesc))]
pub struct GameInputToControlInputSystem {
    /// Reader ID for the `InputEvent` channel.
    #[system_desc(event_channel_reader)]
    input_event_rid: ReaderId<GameInputEvent>,
    /// Pre-allocated vector
    #[new(value = "Vec::with_capacity(64)")]
    #[system_desc(skip)]
    control_input_events: Vec<ControlInputEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GameInputToControlInputSystemData<'s> {
    /// `GameInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_input_ec: Read<'s, EventChannel<GameInputEvent>>,
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: ReadStorage<'s, SharedInputControlled>,
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_game_input_ec: Write<'s, EventChannel<ControlInputEvent>>,
}

impl<'s> System<'s> for GameInputToControlInputSystem {
    type SystemData = GameInputToControlInputSystemData<'s>;

    fn run(
        &mut self,
        GameInputToControlInputSystemData {
            game_input_ec,
            entities,
            input_controlleds,
            shared_input_controlleds,
            mut control_game_input_ec,
        }: Self::SystemData,
    ) {
        game_input_ec
            .read(&mut self.input_event_rid)
            .for_each(|ev| {
                match *ev {
                    GameInputEvent::ActionPressed(PlayerActionControl { player, action }) => {
                        // Find the entity has the `player` control id in its `InputControlled`
                        // component.

                        let shared_input_controlled_entities =
                            (&entities, &shared_input_controlleds)
                                .join()
                                .map(|(entity, _)| entity);

                        let control_input_events_iter = (&entities, &input_controlleds)
                            .join()
                            .filter_map(|(entity, input_controlled)| {
                                if input_controlled.controller_id == player {
                                    Some(entity)
                                } else {
                                    None
                                }
                            })
                            .chain(shared_input_controlled_entities)
                            .map(|entity| {
                                ControlInputEvent::ControlActionPress(ControlActionEventData {
                                    controller_id: player,
                                    entity,
                                    control_action: action,
                                })
                            });

                        self.control_input_events.extend(control_input_events_iter);
                    }
                    GameInputEvent::ActionReleased(PlayerActionControl { player, action }) => {
                        let shared_input_controlled_entities =
                            (&entities, &shared_input_controlleds)
                                .join()
                                .map(|(entity, _)| entity);

                        let control_input_events_iter = (&entities, &input_controlleds)
                            .join()
                            .filter_map(|(entity, input_controlled)| {
                                if input_controlled.controller_id == player {
                                    Some(entity)
                                } else {
                                    None
                                }
                            })
                            .chain(shared_input_controlled_entities)
                            .map(|entity| {
                                ControlInputEvent::ControlActionRelease(ControlActionEventData {
                                    controller_id: player,
                                    entity,
                                    control_action: action,
                                })
                            });

                        self.control_input_events.extend(control_input_events_iter);
                    }
                    GameInputEvent::AxisMoved {
                        axis: PlayerAxisControl { player, axis },
                        value,
                    } => {
                        let shared_input_controlled_entities =
                            (&entities, &shared_input_controlleds)
                                .join()
                                .map(|(entity, _)| entity);

                        let control_input_events_iter = (&entities, &input_controlleds)
                            .join()
                            .filter_map(|(entity, input_controlled)| {
                                if input_controlled.controller_id == player {
                                    Some(entity)
                                } else {
                                    None
                                }
                            })
                            .chain(shared_input_controlled_entities)
                            .map(|entity| {
                                ControlInputEvent::AxisMoved(AxisMoveEventData {
                                    controller_id: player,
                                    entity,
                                    axis,
                                    value,
                                })
                            });

                        self.control_input_events.extend(control_input_events_iter);
                    }
                }
            });

        control_game_input_ec.drain_vec_write(&mut self.control_input_events);
    }
}

use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, System, World, Write},
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        // TODO: figure out how to implement controller configuration updates, because we need to
        // update the resource and what this system stores.
        world.insert(self.input_config.clone());

        self.input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<InputEvent<ControlBindings>>>()
                .register_reader(),
        );
    }
}

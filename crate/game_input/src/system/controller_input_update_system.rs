use amethyst::{
    ecs::{Component, Entity, Read, System, SystemData, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use derive_new::new;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use typename_derive::TypeName;

use crate::ControllerInput;

/// Updates `ControllerInput` based on input events.
#[derive(Debug, Default, TypeName, new)]
pub struct ControllerInputUpdateSystem {
    /// Reader ID for the `ControlInputEvent` event channel.
    #[new(default)]
    input_events_id: Option<ReaderId<ControlInputEvent>>,
}

type ControllerInputUpdateSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    WriteStorage<'s, ControllerInput>,
);

impl ControllerInputUpdateSystem {
    fn get_or_insert_mut<'s, C>(comp_storage: &'s mut WriteStorage<C>, entity: Entity) -> &'s mut C
    where
        C: Component + Default,
    {
        if let Ok(entry) = comp_storage.entry(entity) {
            entry.or_insert(C::default());
        }
        comp_storage
            .get_mut(entity)
            .expect("Unreachable: Component either previously existed, or was just inserted.")
    }
}

impl<'s> System<'s> for ControllerInputUpdateSystem {
    type SystemData = ControllerInputUpdateSystemData<'s>;

    fn run(&mut self, (input_events, mut controller_inputs): Self::SystemData) {
        let input_events_id = self
            .input_events_id
            .as_mut()
            .expect("Expected `input_events_id` field to be set.");

        input_events.read(input_events_id).for_each(|ev| match ev {
            ControlInputEvent::AxisMoved(AxisMoveEventData {
                entity,
                axis,
                value,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match axis {
                    Axis::X => controller_input.x_axis_value = *value,
                    Axis::Z => controller_input.z_axis_value = *value,
                };
            }
            ControlInputEvent::ControlActionPress(ControlActionEventData {
                entity,
                control_action,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match control_action {
                    ControlAction::Defend => controller_input.defend = true,
                    ControlAction::Jump => controller_input.jump = true,
                    ControlAction::Attack => controller_input.attack = true,
                    ControlAction::Special => controller_input.special = true,
                };
            }
            ControlInputEvent::ControlActionRelease(ControlActionEventData {
                entity,
                control_action,
            }) => {
                let controller_input = Self::get_or_insert_mut(&mut controller_inputs, *entity);
                match control_action {
                    ControlAction::Defend => controller_input.defend = false,
                    ControlAction::Jump => controller_input.jump = false,
                    ControlAction::Attack => controller_input.attack = false,
                    ControlAction::Special => controller_input.special = false,
                };
            }
        });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.input_events_id = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
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
    use typename::TypeName;

    use super::ControllerInputUpdateSystem;
    use crate::ControllerInput;

    #[test]
    fn inserts_controller_input_if_non_existent() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                ControllerInputUpdateSystem::new(),
                ControllerInputUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(|world| {
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
                ControllerInputUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(|world| {
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

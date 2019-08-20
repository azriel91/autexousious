use amethyst::ecs::prelude::*;
use derive_new::new;
use typename_derive::TypeName;

use crate::{ControllerInput, InputControlled, SharedInputControlled};

/// Updates the `ControllerInput` component based on input from the sharing controllers.
#[derive(Debug, Default, TypeName, new)]
pub struct SharedControllerInputUpdateSystem;

type SharedControllerInputUpdateSystemData<'s> = (
    ReadStorage<'s, InputControlled>,
    WriteStorage<'s, ControllerInput>,
    ReadStorage<'s, SharedInputControlled>,
    Entities<'s>,
);

impl<'s> System<'s> for SharedControllerInputUpdateSystem {
    type SystemData = SharedControllerInputUpdateSystemData<'s>;

    fn run(
        &mut self,
        (input_controlleds, mut controller_inputs, shared_input_controlleds,  entities): Self::SystemData,
    ) {
        let mut merged_input = (&input_controlleds, &controller_inputs).join().fold(
            ControllerInput::default(),
            |mut merged, (_, controller_input)| {
                merged.x_axis_value += controller_input.x_axis_value;
                merged.z_axis_value += controller_input.z_axis_value;
                merged.defend |= controller_input.defend;
                merged.jump |= controller_input.jump;
                merged.attack |= controller_input.attack;
                merged.special |= controller_input.special;

                merged
            },
        );

        if merged_input.x_axis_value < -1. {
            merged_input.x_axis_value = -1.;
        } else if merged_input.x_axis_value > 1. {
            merged_input.x_axis_value = 1.;
        }

        if merged_input.z_axis_value < -1. {
            merged_input.z_axis_value = -1.;
        } else if merged_input.z_axis_value > 1. {
            merged_input.z_axis_value = 1.;
        }

        for (entity, _) in (&*entities, &shared_input_controlleds).join() {
            controller_inputs
                .insert(entity, merged_input)
                // kcov-ignore-start
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to replace `{}`. Error: `{}`",
                        stringify!(ControllerInput),
                        e
                    )
                });
            // kcov-ignore-end
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, Join, ReadStorage, WorldExt, WriteStorage},
        Error,
    };
    use amethyst_test::*;
    use game_input_model::{ControlBindings, ControllerId};
    use typename::TypeName;

    use super::SharedControllerInputUpdateSystem;
    use crate::{ControllerInput, InputControlled, SharedInputControlled};

    #[test]
    fn merges_axes_controller_input_with_limit_correction() -> Result<(), Error> {
        let controller_count = 3;
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                SharedControllerInputUpdateSystem::new(),
                SharedControllerInputUpdateSystem::type_name(),
                &[],
            )
            .with_setup(move |world| {
                let controller_entities = (0..controller_count)
                    .map(|n| {
                        let controller_id = n as ControllerId;
                        world
                            .create_entity()
                            .with(InputControlled::new(controller_id))
                            .with(ControllerInput::default())
                            .build()
                    })
                    .collect::<Vec<Entity>>();
                world.insert(EffectReturn(controller_entities));

                let entity = world.create_entity().with(SharedInputControlled).build();
                world.insert(EffectReturn(entity));
            })
            .with_assertion(|world| {
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., false, false, false, false)),
                    store.join().next()
                );
            })
            .with_effect(|world| {
                world.exec(
                    |(input_controlleds, mut controller_inputs): (
                        ReadStorage<'_, InputControlled>,
                        WriteStorage<'_, ControllerInput>,
                    )| {
                        (&input_controlleds, &mut controller_inputs)
                            .join()
                            .for_each(|(_, controller_input)| {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = 1.;
                            });
                    }, // kcov-ignore
                );
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(-1., 1., false, false, false, false)),
                    store.get(entity)
                );
            })
            .with_effect(|world| {
                world.exec(
                    |(input_controlleds, mut controller_inputs): (
                        ReadStorage<'_, InputControlled>,
                        WriteStorage<'_, ControllerInput>,
                    )| {
                        (&input_controlleds, &mut controller_inputs)
                            .join()
                            .for_each(|(_, controller_input)| {
                                controller_input.x_axis_value = 1.;
                                controller_input.z_axis_value = -1.;
                            });
                    }, // kcov-ignore
                );
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(1., -1., false, false, false, false)),
                    store.get(entity)
                );
            })
            .with_effect(|world| {
                let controller_entities = (world.read_resource::<EffectReturn<Vec<Entity>>>().0)
                    .iter()
                    .map(|e| *e)
                    .collect::<Vec<_>>();

                controller_entities.into_iter().for_each(|entity| {
                    world
                        .delete_entity(entity)
                        .expect("Failed to delete entity.")
                });
            })
            .with_assertion(|world| {
                // Make sure it unsets the control input when released
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., false, false, false, false)),
                    store.get(entity)
                );
            })
            .run()
    }

    #[test]
    fn merges_action_controller_input() -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                SharedControllerInputUpdateSystem::new(),
                SharedControllerInputUpdateSystem::type_name(),
                &[],
            )
            .with_setup(|world| {
                let entity = world.create_entity().with(SharedInputControlled).build();
                world.insert(EffectReturn(entity));
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., false, false, false, false)),
                    store.get(entity)
                );
            })
            .with_effect(|world| {
                let mut jump_attack_pressed = ControllerInput::default();
                jump_attack_pressed.attack = true;
                jump_attack_pressed.jump = true;

                let entity_0 = world
                    .create_entity()
                    .with(InputControlled::new(0))
                    .with(jump_attack_pressed)
                    .build();

                let mut defend_special_pressed = ControllerInput::default();
                defend_special_pressed.defend = true;
                defend_special_pressed.special = true;

                let entity_1 = world
                    .create_entity()
                    .with(InputControlled::new(0))
                    .with(defend_special_pressed)
                    .build();

                let controller_entities = vec![entity_0, entity_1];
                world.insert(EffectReturn(controller_entities));
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., true, true, true, true)),
                    store.get(entity)
                );
            })
            .with_effect(|world| {
                let entities = (world.read_resource::<EffectReturn<Vec<Entity>>>().0)
                    .iter()
                    .map(|e| *e)
                    .collect::<Vec<_>>();

                world
                    .delete_entity(*entities.first().expect("Expected entity to exist."))
                    .expect("Failed to delete `jump_attack` entity.");
            })
            .with_assertion(|world| {
                // Make sure it unsets the control input when released
                let store = world.read_storage::<ControllerInput>();
                assert_eq!(
                    Some(&ControllerInput::new(0., 0., true, false, false, true)),
                    store.join().next()
                );
            })
            .run()
    }
}

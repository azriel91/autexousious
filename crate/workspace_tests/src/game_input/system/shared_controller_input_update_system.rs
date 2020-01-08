#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Builder, Entity, Join, ReadStorage, WorldExt, WriteStorage},
        Error,
    };
    use amethyst_test::{AmethystApplication, EffectReturn};
    use game_input_model::{ControlBindings, ControllerId};

    use game_input::{
        ControllerInput, InputControlled, SharedControllerInputUpdateSystem, SharedInputControlled,
    };

    #[test]
    fn merges_axes_controller_input_with_limit_correction() -> Result<(), Error> {
        let controller_count = 3;
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                SharedControllerInputUpdateSystem::new(),
                any::type_name::<SharedControllerInputUpdateSystem>(),
                &[],
            )
            .with_effect(move |world| {
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
                any::type_name::<SharedControllerInputUpdateSystem>(),
                &[],
            )
            .with_effect(|world| {
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

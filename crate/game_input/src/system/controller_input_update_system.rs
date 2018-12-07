use amethyst::{ecs::prelude::*, input::InputHandler};

use crate::{
    Axis, ControlAction, ControllerInput, InputConfig, InputControlled, PlayerActionControl,
    PlayerAxisControl,
};

/// Updates `Character` sequence based on input
#[derive(Debug, Default, TypeName, new)]
pub struct ControllerInputUpdateSystem {
    /// All controller input configuration.
    input_config: InputConfig,
}

type ControllerInputUpdateSystemData<'s> = (
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    ReadStorage<'s, InputControlled>,
    Entities<'s>,
    WriteStorage<'s, ControllerInput>,
);

impl<'s> System<'s> for ControllerInputUpdateSystem {
    type SystemData = ControllerInputUpdateSystemData<'s>;

    fn run(
        &mut self,
        (
            input_handler,
            input_controlleds,
            entities,
            mut controller_input_storage,
): Self::SystemData,
    ) {
        for (entity, input_controlled) in (&*entities, &input_controlleds).join() {
            let player = input_controlled.controller_id;

            let x_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::X));
            let z_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::Z));

            let input = ControllerInput::new(
                x_axis_value.unwrap_or(0.),
                z_axis_value.unwrap_or(0.),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Defend))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Jump))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Attack))
                    .unwrap_or(false),
                input_handler
                    .action_is_down(&PlayerActionControl::new(player, ControlAction::Special))
                    .unwrap_or(false),
            );

            controller_input_storage
                .insert(entity, input)
                .expect("Failed to replace `ControllerInput`.");
        }
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
    use amethyst::ecs::prelude::*;
    use amethyst_test::*;
    use typename::TypeName;

    use super::ControllerInputUpdateSystem;
    use crate::ControllerInput;
    use crate::InputConfig;
    use crate::InputControlled;
    use crate::PlayerActionControl;
    use crate::PlayerAxisControl;

    #[test]
    fn updates_controller_input_from_input_bindings() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_system(
                    ControllerInputUpdateSystem::new(InputConfig::default()),
                    ControllerInputUpdateSystem::type_name(),
                    &[]
                )
                .with_setup(|world| {
                    let controller_id = 0;
                    let entity = world
                        .create_entity()
                        .with(InputControlled::new(controller_id))
                        .build();

                    world.add_resource(EffectReturn(entity));
                })
                .with_assertion(|world| {
                    let entity = world.read_resource::<EffectReturn<Entity>>().0;
                    let store = world.read_storage::<ControllerInput>();
                    assert_eq!(
                        Some(&ControllerInput::new(0., 0., false, false, false, false)),
                        store.get(entity)
                    );
                })
                .run()
                .is_ok()
        );
    }
}

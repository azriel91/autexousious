use amethyst::{ecs::prelude::*, input::InputHandler};
use character_selection::CharacterEntityControl;
use game_input::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl};
use object_model::entity::CharacterInput;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterInputUpdateSystem;

type CharacterInputUpdateSystemData<'s> = (
    Entities<'s>,
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    ReadStorage<'s, CharacterEntityControl>,
    WriteStorage<'s, CharacterInput>,
);

impl<'s> System<'s> for CharacterInputUpdateSystem {
    type SystemData = CharacterInputUpdateSystemData<'s>;

    fn run(
        &mut self,
        (entities, input_handler, control_storage, mut character_input_storage): Self::SystemData,
    ) {
        for (entity, character_entity_control) in (&*entities, &control_storage).join() {
            let player = character_entity_control.controller_id;

            let x_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::X));
            let z_axis_value = input_handler.axis_value(&PlayerAxisControl::new(player, Axis::Z));

            let input = CharacterInput::new(
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

            character_input_storage
                .insert(entity, input)
                .expect("Failed to replace `CharacterInput` for character.");
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst::ecs::prelude::*;
    use amethyst_test_support::*;
    use application::resource::dir::ASSETS;
    use application_test_support::AutexousiousApplication;
    use character_selection::CharacterEntityControl;
    use loading::LoadingState;
    use object_model::entity::CharacterInput;
    use typename::TypeName;

    use super::CharacterInputUpdateSystem;

    #[test]
    fn maintains_character_sequence_when_next_sequence_is_none() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "maintains_character_sequence_when_next_sequence_is_none",
                false
            ).with_system(
                CharacterInputUpdateSystem::new(),
                CharacterInputUpdateSystem::type_name(),
                &[]
            ).with_setup(|world| {
                let controller_id = 0;
                let entity = world
                    .create_entity()
                    .with(CharacterEntityControl::new(controller_id))
                    .build();

                world.add_resource(EffectReturn(entity));
            }).with_state(|| LoadingState::new(
                Path::new(env!("CARGO_MANIFEST_DIR")).join(ASSETS),
                Box::new(EmptyState),
            )).with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                let store = world.read_storage::<CharacterInput>();
                assert_eq!(
                    Some(&CharacterInput::new(0., 0., false, false, false, false)),
                    store.get(entity)
                );
            }).run()
            .is_ok()
        );
    }
}

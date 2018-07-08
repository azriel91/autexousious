use amethyst::{ecs::prelude::*, input::InputHandler};
use character_selection::CharacterEntityControl;
use game_input::{Axis, ControlAction, PlayerActionControl, PlayerAxisControl};
use object_model::entity::CharacterInput;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
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
    use std::env;

    use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::Material};
    use amethyst_test_support::*;
    use application_test_support::AutexousiousApplication;
    use character_selection::CharacterEntityControl;
    use game_play_state::GamePlayState;
    use loading;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{CharacterInput, Kinematics, ObjectStatus},
        loaded::CharacterHandle,
    };

    use super::CharacterInputUpdateSystem;

    #[test]
    #[ignore]
    fn maintains_character_sequence_when_next_sequence_is_none() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let setup = |world: &mut World| {
            // Create entity with CharacterEntityControl
            let controller_id = 0;
            let entity = world
                .create_entity()
                .with(CharacterEntityControl::new(controller_id))
                .build();

            world.add_resource(EffectReturn(entity));
        };

        let load_and_play_state = || {
            loading::State::new(
                AmethystApplication::assets_dir().into(),
                Box::new(GamePlayState::new()),
            )
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            let store = world.read_storage::<CharacterInput>();
            assert_eq!(
                Some(&CharacterInput::new(0., 0., false, false, false, false)),
                store.get(entity)
            );
        };

        assert!(
            AutexousiousApplication::game_base(
                "maintains_character_sequence_when_next_sequence_is_none",
                false
            ).with_system(TestSystem, "test_system", &[])
                .with_system(
                    CharacterInputUpdateSystem::new(),
                    "character_input_update_system",
                    &[]
                )
                .with_setup(setup)
                .with_state(load_and_play_state)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    // Sets up storages for the various `Component`.
    #[derive(Debug)]
    struct TestSystem;
    type TestSystemData<'s> = (
        ReadStorage<'s, CharacterHandle>,
        ReadStorage<'s, Kinematics<f32>>,
        ReadStorage<'s, ObjectStatus<CharacterSequenceId>>,
        ReadStorage<'s, AnimationControlSet<CharacterSequenceId, Material>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {
            panic!(
                "TODO: Stop the `GamePlayState` after one cycle, but not delete the entities so \
                 that we can do the assertion."
            );
        }
    }
}

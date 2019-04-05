use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use character_model::{config::CharacterSequenceId, loaded::Character};
use derive_new::new;
use game_input::ControllerInput;
use named_type::NamedType;
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{
    CharacterGroundingSystem, CharacterHitEffectSystem, CharacterKinematicsSystem,
    CharacterSequenceUpdateSystem, ComponentSequencesUpdateSystem, FrameComponentUpdateSystem,
    GamePlayEndDetectionSystem, GamePlayEndTransitionSystem, ObjectCollisionDetectionSystem,
    ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem, SequenceUpdateSystem,
};

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterKinematicsSystem::new(),
            &CharacterKinematicsSystem::type_name(),
            &[&CharacterSequenceUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&CharacterKinematicsSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterGroundingSystem::new(),
            &CharacterGroundingSystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        ); // kcov-ignore

        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[&CharacterGroundingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectCollisionDetectionSystem::new(),
            &ObjectCollisionDetectionSystem::type_name(),
            &[&ObjectTransformUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterHitEffectSystem::new(),
            &CharacterHitEffectSystem::type_name(),
            &[&ObjectCollisionDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            GamePlayEndDetectionSystem::new(),
            &GamePlayEndDetectionSystem::type_name(),
            &[&CharacterHitEffectSystem::type_name()],
        ); // kcov-ignore

        // TODO: autogenerate these
        builder.add(
            ComponentSequencesUpdateSystem::<Character>::new(),
            &ComponentSequencesUpdateSystem::<Character>::type_name(),
            &[
                &CharacterSequenceUpdateSystem::type_name(),
                &CharacterHitEffectSystem::type_name(),
            ],
        ); // kcov-ignore

        builder.add(
            SequenceUpdateSystem::new(),
            &SequenceUpdateSystem::type_name(),
            &[&ComponentSequencesUpdateSystem::<Character>::type_name()],
        ); // kcov-ignore
        builder.add(
            FrameComponentUpdateSystem::new(),
            &FrameComponentUpdateSystem::type_name(),
            &[&SequenceUpdateSystem::type_name()],
        ); // kcov-ignore

        // Depends on the LastTrackerSystem<ControllerInput>, so must run before it.
        builder.add(
            GamePlayEndTransitionSystem::new(),
            &GamePlayEndTransitionSystem::type_name(),
            &[],
        ); // kcov-ignore

        // === `LastTrackerSystem`s === //

        let controller_input_tracker_system =
            LastTrackerSystem::<ControllerInput>::new(stringify!(game_input::ControllerInput));
        let controller_input_tracker_system_name = controller_input_tracker_system.system_name();

        // This depends on `&ControllerInputUpdateSystem::type_name()`, but since it runs in a
        // separate dispatcher, we have to omit it from here.
        builder.add(
            controller_input_tracker_system,
            &controller_input_tracker_system_name,
            &[&GamePlayEndTransitionSystem::type_name()],
        ); // kcov-ignore

        let character_sequence_id_tracker_system =
            LastTrackerSystem::<CharacterSequenceId>::new(stringify!(CharacterSequenceId));
        let character_sequence_id_tracker_system_name =
            character_sequence_id_tracker_system.system_name();
        builder.add(
            character_sequence_id_tracker_system,
            &character_sequence_id_tracker_system_name,
            &[&GamePlayEndTransitionSystem::type_name()],
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input_model::{PlayerActionControl, PlayerAxisControl};

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(GamePlayBundle::new())
                .run()
                .is_ok()
        );
    }
}

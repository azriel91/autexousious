use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use game_input::ControllerInput;
use named_type::NamedType;
use object_model::{config::object::CharacterSequenceId, entity::ObjectStatus};
use object_play::RunCounterUpdateSystem;
use tracker::LastTrackerSystem;
use typename::TypeName;

use CharacterCollisionEffectSystem;
use CharacterGroundingSystem;
use CharacterKinematicsSystem;
use CharacterSequenceUpdateSystem;
use GamePlayEndDetectionSystem;
use GamePlayEndTransitionSystem;
use ObjectAnimationUpdateSystem;
use ObjectCollisionDetectionSystem;
use ObjectKinematicsUpdateSystem;
use ObjectTransformUpdateSystem;

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            RunCounterUpdateSystem::new(),
            &RunCounterUpdateSystem::type_name(),
            &[&CharacterSequenceUpdateSystem::type_name()],
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
            CharacterCollisionEffectSystem::new(),
            &CharacterCollisionEffectSystem::type_name(),
            &[&ObjectCollisionDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            GamePlayEndDetectionSystem::new(),
            &GamePlayEndDetectionSystem::type_name(),
            &[&CharacterCollisionEffectSystem::type_name()],
        ); // kcov-ignore

        // Depends on the LastTrackerSystem<ObjectStatus<CharacterSequenceId>>, so must run before it.
        builder.add(
            ObjectAnimationUpdateSystem::<CharacterSequenceId>::new(),
            &ObjectAnimationUpdateSystem::<CharacterSequenceId>::type_name(),
            &[&CharacterCollisionEffectSystem::type_name()],
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

        let character_object_status_tracker_system =
            LastTrackerSystem::<ObjectStatus<CharacterSequenceId>>::new(stringify!(
                ObjectStatus<CharacterSequenceId>
            ));
        let character_object_status_tracker_system_name =
            character_object_status_tracker_system.system_name();
        builder.add(
            character_object_status_tracker_system,
            &character_object_status_tracker_system_name,
            &[&ObjectAnimationUpdateSystem::<CharacterSequenceId>::type_name()],
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};

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

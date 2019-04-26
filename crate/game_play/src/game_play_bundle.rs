use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use character_model::{config::CharacterSequenceId, loaded::Character};
use character_play::{
    CharacterControlTransitionsTransitionSystem, CharacterControlTransitionsUpdateSystem,
    CharacterCtsHandleUpdateSystem,
};
use collision_play::{
    HitDetectionSystem, HitRepeatTrackersAugmentSystem, HitRepeatTrackersTickerSystem,
};
use derive_new::new;
use game_input::ControllerInput;
use named_type::NamedType;
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{
    CharacterGroundingSystem, CharacterHitEffectSystem, CharacterKinematicsSystem,
    CharacterSequenceUpdateSystem, ComponentSequencesUpdateSystem, FrameComponentUpdateSystem,
    FrameFreezeClockAugmentSystem, GamePlayEndDetectionSystem, GamePlayEndTransitionSystem,
    ObjectCollisionDetectionSystem, ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
    SequenceUpdateSystem,
};

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // === Component augmentation === //

        builder.add(
            ComponentSequencesUpdateSystem::<Character>::new(),
            &ComponentSequencesUpdateSystem::<Character>::type_name(),
            &[],
        ); // kcov-ignore

        // Updates frame limit and ticks the sequence logic clocks.
        builder.add(
            SequenceUpdateSystem::new(),
            &SequenceUpdateSystem::type_name(),
            &[&ComponentSequencesUpdateSystem::<Character>::type_name()],
        ); // kcov-ignore
        builder.add(
            FrameComponentUpdateSystem::new(),
            &FrameComponentUpdateSystem::type_name(),
            &[
                &ComponentSequencesUpdateSystem::<Character>::type_name(),
                &SequenceUpdateSystem::type_name(),
            ],
        ); // kcov-ignore
        builder.add(
            CharacterCtsHandleUpdateSystem::new(),
            &CharacterCtsHandleUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterControlTransitionsUpdateSystem::new(),
            &CharacterControlTransitionsUpdateSystem::type_name(),
            &[&CharacterCtsHandleUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            FrameFreezeClockAugmentSystem::new(),
            &FrameFreezeClockAugmentSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            HitRepeatTrackersAugmentSystem::new(),
            &HitRepeatTrackersAugmentSystem::type_name(),
            &[],
        ); // kcov-ignore

        builder.add_barrier();

        // === Component value update === //

        // Sets velocity based on sequence ID and input.
        builder.add(
            CharacterKinematicsSystem::new(),
            &CharacterKinematicsSystem::type_name(),
            &[],
        ); // kcov-ignore
           // pos += vel
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&CharacterKinematicsSystem::type_name()],
        ); // kcov-ignore

        // `Position` correction based on margins.
        builder.add(
            CharacterGroundingSystem::new(),
            &CharacterGroundingSystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[
                &ObjectKinematicsUpdateSystem::type_name(),
                &CharacterGroundingSystem::type_name(),
            ],
        ); // kcov-ignore
        builder.add(
            HitRepeatTrackersTickerSystem::new(),
            &HitRepeatTrackersTickerSystem::type_name(),
            &[&HitRepeatTrackersAugmentSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Effect Detection === //

        builder.add(
            ObjectCollisionDetectionSystem::new(),
            &ObjectCollisionDetectionSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            HitDetectionSystem::new(),
            &HitDetectionSystem::type_name(),
            &[&ObjectCollisionDetectionSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Sequence ID Updates === //

        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterControlTransitionsTransitionSystem::new(),
            &CharacterControlTransitionsTransitionSystem::type_name(),
            &[&CharacterSequenceUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterHitEffectSystem::new(),
            &CharacterHitEffectSystem::type_name(),
            &[&CharacterControlTransitionsTransitionSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Helper Systems === //

        // Detects when the winning condition has been met.
        builder.add(
            GamePlayEndDetectionSystem::new(),
            &GamePlayEndDetectionSystem::type_name(),
            &[],
        ); // kcov-ignore
           // Sends a state transition when game play ends, and `Attack` is pressed.
        builder.add(
            GamePlayEndTransitionSystem::new(),
            &GamePlayEndTransitionSystem::type_name(),
            &[&GamePlayEndDetectionSystem::type_name()],
        ); // kcov-ignore

        let controller_input_tracker_system =
            LastTrackerSystem::<ControllerInput>::new(stringify!(game_input::ControllerInput));
        let controller_input_tracker_system_name = controller_input_tracker_system.system_name();
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
            &[],
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input_model::{PlayerActionControl, PlayerAxisControl};

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
            .with_bundle(GamePlayBundle::new())
            .run()
    }
}

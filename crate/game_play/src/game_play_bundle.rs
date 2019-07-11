use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use character_model::{config::CharacterSequenceId, loaded::Character};
use character_play::{
    CharacterControlTransitionsTransitionSystem, CharacterControlTransitionsUpdateSystem,
    CharacterCtsHandleUpdateSystem,
};
use chase_play::StickToTargetObjectSystem;
use collision_audio_play::HitSfxSystem;
use collision_model::loaded::{BodySequence, InteractionsSequence};
use collision_play::{
    CollisionDetectionSystem, ContactDetectionSystem, HitDetectionSystem,
    HitRepeatTrackersAugmentSystem, HitRepeatTrackersTickerSystem,
};
use derive_new::new;
use energy_model::{config::EnergySequenceId, loaded::Energy};
use energy_play::{EnergyHitEffectSystem, EnergyHittingEffectSystem};
use game_input::ControllerInput;
use game_play_hud::HpBarUpdateSystem;
use map_model::config::MapLayerSequenceId;
use named_type::NamedType;
use object_play::{ObjectGravitySystem, ObjectMirroringSystem};
use object_status_play::StunPointsReductionSystem;
use sequence_model::loaded::WaitSequence;
use sequence_play::{
    FrameComponentUpdateSystem, SequenceEndTransitionSystem, SequenceStatusUpdateSystem,
    SequenceUpdateSystem,
};
use spawn_model::loaded::SpawnsSequence;
use spawn_play::{SpawnGameObjectRectifySystem, SpawnGameObjectSystem};
use sprite_model::loaded::SpriteRenderSequence;
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{
    CharacterHitEffectSystem, CharacterKinematicsSystem, CharacterSequenceUpdateSystem,
    FrameFreezeClockAugmentSystem, GamePlayEndDetectionSystem, GamePlayEndTransitionSystem,
    GamePlayRemovalAugmentSystem, ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
    SequenceComponentUpdateSystem,
};

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // === Component augmentation === //

        macro_rules! add_frame_component_update_system {
            ($frame_component_data:ident) => {
                builder.add(
                    FrameComponentUpdateSystem::<$frame_component_data>::new(),
                    &FrameComponentUpdateSystem::<$frame_component_data>::type_name(),
                    &[
                        &SequenceComponentUpdateSystem::<Character>::type_name(),
                        &SequenceComponentUpdateSystem::<Energy>::type_name(),
                        &SequenceUpdateSystem::type_name(),
                    ],
                ); // kcov-ignore
            };
        }

        builder.add(
            SequenceComponentUpdateSystem::<Character>::new(),
            &SequenceComponentUpdateSystem::<Character>::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            SequenceComponentUpdateSystem::<Energy>::new(),
            &SequenceComponentUpdateSystem::<Energy>::type_name(),
            &[],
        ); // kcov-ignore

        macro_rules! sequence_status_update_system {
            ($sequence_id_type:ty) => {
                builder.add(
                    SequenceStatusUpdateSystem::<$sequence_id_type>::new(),
                    &SequenceStatusUpdateSystem::<$sequence_id_type>::type_name(),
                    &[],
                ); // kcov-ignore
            };
        }
        sequence_status_update_system!(MapLayerSequenceId);
        sequence_status_update_system!(CharacterSequenceId);
        sequence_status_update_system!(EnergySequenceId);

        // Updates frame limit and ticks the sequence logic clocks.
        builder.add(
            SequenceUpdateSystem::new(),
            &SequenceUpdateSystem::type_name(),
            &[
                &SequenceStatusUpdateSystem::<MapLayerSequenceId>::type_name(),
                &SequenceStatusUpdateSystem::<CharacterSequenceId>::type_name(),
                &SequenceStatusUpdateSystem::<EnergySequenceId>::type_name(),
            ],
        ); // kcov-ignore
        add_frame_component_update_system!(WaitSequence);
        add_frame_component_update_system!(SpriteRenderSequence);
        add_frame_component_update_system!(BodySequence);
        add_frame_component_update_system!(InteractionsSequence);
        add_frame_component_update_system!(SpawnsSequence);
        builder.add(
            CharacterCtsHandleUpdateSystem::new(),
            &CharacterCtsHandleUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterControlTransitionsUpdateSystem::new(),
            &CharacterControlTransitionsUpdateSystem::type_name(),
            &[
                &CharacterCtsHandleUpdateSystem::type_name(),
                &SequenceUpdateSystem::type_name(),
            ],
        ); // kcov-ignore
        builder.add(
            FrameFreezeClockAugmentSystem::new(),
            &FrameFreezeClockAugmentSystem::type_name(),
            &[&SequenceUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            HitRepeatTrackersAugmentSystem::new(),
            &HitRepeatTrackersAugmentSystem::type_name(),
            &[],
        ); // kcov-ignore

        builder.add(HitSfxSystem::new(), &HitSfxSystem::type_name(), &[]);

        // Spawn objects
        builder.add(
            SpawnGameObjectSystem::new(),
            &SpawnGameObjectSystem::type_name(),
            &[&FrameComponentUpdateSystem::<SpawnsSequence>::type_name()],
        ); // kcov-ignore
        builder.add(
            SpawnGameObjectRectifySystem::new(),
            &SpawnGameObjectRectifySystem::type_name(),
            &[&SpawnGameObjectSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            GamePlayRemovalAugmentSystem::new(),
            &GamePlayRemovalAugmentSystem::type_name(),
            &[&SpawnGameObjectSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Component value update === //

        // Sets velocity based on sequence ID and input.
        builder.add(
            CharacterKinematicsSystem::new(),
            &CharacterKinematicsSystem::type_name(),
            &[],
        ); // kcov-ignore

        // Reduces `StunPoints` each tick.
        builder.add(
            StunPointsReductionSystem::new(),
            &StunPointsReductionSystem::type_name(),
            &[],
        ); // kcov-ignore

        // pos += vel
        // This must be between the `FrameFreezeClockAugmentSystem` and `SequenceUpdateSystem`s
        // since it needs to wait for the `FrameFreezeClock` to tick.
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&CharacterKinematicsSystem::type_name()],
        ); // kcov-ignore

        // `Position` correction based on margins.
        builder.add(
            ObjectGravitySystem::new(),
            &ObjectGravitySystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[
                &ObjectKinematicsUpdateSystem::type_name(),
                &ObjectGravitySystem::type_name(),
            ],
        ); // kcov-ignore
        builder.add(
            ObjectMirroringSystem::new(),
            &ObjectMirroringSystem::type_name(),
            &[&ObjectTransformUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            StickToTargetObjectSystem::new(),
            &StickToTargetObjectSystem::type_name(),
            &[&ObjectTransformUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            HitRepeatTrackersTickerSystem::new(),
            &HitRepeatTrackersTickerSystem::type_name(),
            &[&HitRepeatTrackersAugmentSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Effect Detection === //

        builder.add(
            CollisionDetectionSystem::new(),
            &CollisionDetectionSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            ContactDetectionSystem::new(),
            &ContactDetectionSystem::type_name(),
            &[&CollisionDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            HitDetectionSystem::new(),
            &HitDetectionSystem::type_name(),
            &[&ContactDetectionSystem::type_name()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Sequence ID Updates === //

        builder.add(
            SequenceEndTransitionSystem::<MapLayerSequenceId>::new(),
            &SequenceEndTransitionSystem::<MapLayerSequenceId>::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            SequenceEndTransitionSystem::<CharacterSequenceId>::new(),
            &SequenceEndTransitionSystem::<CharacterSequenceId>::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            SequenceEndTransitionSystem::<EnergySequenceId>::new(),
            &SequenceEndTransitionSystem::<EnergySequenceId>::type_name(),
            &[],
        ); // kcov-ignore

        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        //
        // It also depends on `&SequenceEndTransitionSystem::<CharacterSequenceId>` as the
        // `CharacterSequenceUpdater` transitions should overwrite the `SequenceEndTransition`
        // update.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[&SequenceEndTransitionSystem::<CharacterSequenceId>::type_name()],
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

        // `Energy` hit / hitting effects.
        // There are only two currently, but if there is a timer system, perhaps that should go
        // last.
        // The `EnergyHitEffectSystem` depends on the `EnergyHittingEffectSystem` to ensure the
        // `Hit` sequence is deterministic and overwrites the `Hitting` sequence.
        builder.add(
            EnergyHittingEffectSystem::new(),
            &EnergyHittingEffectSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            EnergyHitEffectSystem::new(),
            &EnergyHitEffectSystem::type_name(),
            &[&EnergyHittingEffectSystem::type_name()],
        ); // kcov-ignore

        // Perhaps this should be straight after the `StickToTargetObjectSystem`, but we put it here
        // so that the renderer will show the HP including the damage dealt this frame, instead of
        // one frame later.
        builder.add(
            HpBarUpdateSystem::new(),
            &HpBarUpdateSystem::type_name(),
            &[&CharacterHitEffectSystem::type_name()],
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

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input_model::ControlBindings;

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GamePlayBundle::new())
            .run()
    }
}

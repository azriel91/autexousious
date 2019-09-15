use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    utils::ortho_camera::CameraOrthoSystem,
    Error,
};
use audio_model::loaded::{SourceSequence, SourceSequenceHandles};
use audio_play::SequenceAudioPlaySystem;
use camera_play::{CameraTrackingSystem, CameraVelocitySystem};
use character_model::loaded::CharacterObjectWrapper;
use character_play::{
    CharacterControlTransitionsTransitionSystem, CharacterControlTransitionsUpdateSystem,
    CharacterCtsHandleUpdateSystem,
};
use charge_play::{
    ChargeIncrementSystem, ChargeInitializeDelaySystem, ChargeInitializeDetectionSystem,
    ChargeRetentionSystem, ChargeUsageSystem,
};
use chase_play::StickToTargetObjectSystem;
use collision_audio_play::HitSfxSystem;
use collision_model::loaded::{
    BodySequence, BodySequenceHandles, InteractionsSequence, InteractionsSequenceHandles,
};
use collision_play::{
    CollisionDetectionSystem, ContactDetectionSystem, HitDetectionSystem,
    HitRepeatTrackersAugmentSystem, HitRepeatTrackersTickerSystem,
};
use derive_new::new;
use energy_model::loaded::EnergyObjectWrapper;
use energy_play::{EnergyHitEffectSystem, EnergyHittingEffectSystem};
use game_input::ControllerInput;
use game_play_hud::{CpBarUpdateSystem, HpBarUpdateSystem};
use kinematic_model::{
    config::Position,
    loaded::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandles},
};
use map_play::{
    KeepWithinMapBoundsSystem, MapEnterExitDetectionSystem, MapOutOfBoundsClockAugmentSystem,
    MapOutOfBoundsDeletionSystem,
};
use named_type::NamedType;
use object_play::{
    ObjectAccelerationSystem, ObjectGravitySystem, ObjectGroundingSystem, ObjectMirroringSystem,
};
use object_status_play::StunPointsReductionSystem;
use sequence_model::loaded::{SequenceEndTransitions, WaitSequence, WaitSequenceHandles};
use sequence_play::{
    FrameComponentUpdateSystem, SequenceComponentUpdateSystem, SequenceEndTransitionSystem,
    SequenceStatusUpdateSystem, SequenceUpdateSystem,
};
use spawn_model::loaded::{SpawnsSequence, SpawnsSequenceHandles};
use spawn_play::{SpawnGameObjectRectifySystem, SpawnGameObjectSystem};
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandles};
use tracker::LastTrackerSystem;
use typename::TypeName;

use crate::{
    CharacterHitEffectSystem, CharacterSequenceUpdateSystem, FrameFreezeClockAugmentSystem,
    GamePlayEndDetectionSystem, GamePlayEndTransitionSystem, GamePlayRemovalAugmentSystem,
    GroundingFrictionSystem, ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
};

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // === Component augmentation === //

        builder.add(
            SequenceStatusUpdateSystem::new(),
            &SequenceStatusUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore

        macro_rules! sequence_component_update_system {
            ($component_asset_type:path, $component_data_type:path) => {
                builder.add(
                                                    SequenceComponentUpdateSystem::<
                                                        $component_asset_type,
                                                        $component_data_type,
                                                    >::new(),
                                                    &SequenceComponentUpdateSystem::<
                                                        $component_asset_type,
                                                        $component_data_type,
                                                    >::type_name(),
                                                    &[&SequenceStatusUpdateSystem::type_name()],
                                                ); // kcov-ignore
            };
        }

        macro_rules! object_sequence_component_update_systems {
            ($wrapper_type:path) => {
                sequence_component_update_system!($wrapper_type, WaitSequenceHandles);
                sequence_component_update_system!($wrapper_type, SourceSequenceHandles);
                sequence_component_update_system!($wrapper_type, ObjectAccelerationSequenceHandles);
                sequence_component_update_system!($wrapper_type, SpriteRenderSequenceHandles);
                sequence_component_update_system!($wrapper_type, BodySequenceHandles);
                sequence_component_update_system!($wrapper_type, InteractionsSequenceHandles);
                sequence_component_update_system!($wrapper_type, SpawnsSequenceHandles);
                sequence_component_update_system!($wrapper_type, SequenceEndTransitions);
            };
        }

        object_sequence_component_update_systems!(CharacterObjectWrapper);
        object_sequence_component_update_systems!(EnergyObjectWrapper);

        // TODO: The `SequenceUpdateSystem`s depend on the following systems:
        //
        // * `SequenceComponentUpdateSystem::<_, _, _>`
        //
        // Because there are so many, and we haven't implemented a good way to specify the
        // dependencies without heaps of duplicated code, we use a barrier.
        //
        // TODO: We can potentially use the `inventory` crate to generate the systems and
        // dependencies.
        builder.add_barrier();

        // Updates frame limit and ticks the sequence logic clocks.
        builder.add(
            SequenceUpdateSystem::new(),
            &SequenceUpdateSystem::type_name(),
            &[
                // &SequenceComponentUpdateSystem::<_, _, _>::type_name(),
            ],
        ); // kcov-ignore

        macro_rules! frame_component_update_system {
            ($frame_component_data:ident) => {
                builder.add(
                    FrameComponentUpdateSystem::<$frame_component_data>::new(),
                    &FrameComponentUpdateSystem::<$frame_component_data>::type_name(),
                    &[&SequenceUpdateSystem::type_name()],
                ); // kcov-ignore
            };
        }
        frame_component_update_system!(WaitSequence);
        frame_component_update_system!(SourceSequence);
        frame_component_update_system!(ObjectAccelerationSequence);
        frame_component_update_system!(SpriteRenderSequence);
        frame_component_update_system!(BodySequence);
        frame_component_update_system!(InteractionsSequence);
        frame_component_update_system!(SpawnsSequence);

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

        // Play sounds from sequence updates.
        builder.add(
            SequenceAudioPlaySystem::new(),
            &SequenceAudioPlaySystem::type_name(),
            &[&FrameComponentUpdateSystem::<SourceSequence>::type_name()],
        ); // kcov-ignore

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

        // vel += `ObjectAcceleration` (from frame config).
        builder.add(
            ObjectAccelerationSystem::new(),
            &ObjectAccelerationSystem::type_name(),
            &[],
        ); // kcov-ignore

        // pos += vel
        // This must be between the `FrameFreezeClockAugmentSystem` and `SequenceUpdateSystem`s
        // since it needs to wait for the `FrameFreezeClock` to tick.
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&ObjectAccelerationSystem::type_name()],
        ); // kcov-ignore

        // `Position` correction based on margins.
        // vel += mass
        builder.add(
            ObjectGravitySystem::new(),
            &ObjectGravitySystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            MapEnterExitDetectionSystem::new(),
            &MapEnterExitDetectionSystem::type_name(),
            &[&ObjectGravitySystem::type_name()],
        ); // kcov-ignore
        builder.add(
            KeepWithinMapBoundsSystem::new(),
            &KeepWithinMapBoundsSystem::type_name(),
            &[&MapEnterExitDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectGroundingSystem::new(),
            &ObjectGroundingSystem::type_name(),
            &[&MapEnterExitDetectionSystem::type_name()],
        ); // kcov-ignore

        // Updates `Velocity<f32>` based on grounding.
        builder.add(
            GroundingFrictionSystem::new(),
            &GroundingFrictionSystem::type_name(),
            &[&ObjectGroundingSystem::type_name()],
        ); // kcov-ignore

        builder.add(
            MapOutOfBoundsDeletionSystem::new(),
            &MapOutOfBoundsDeletionSystem::type_name(),
            &[&MapEnterExitDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            MapOutOfBoundsClockAugmentSystem::new(),
            &MapOutOfBoundsClockAugmentSystem::type_name(),
            &[&MapOutOfBoundsDeletionSystem::type_name()],
        ); // kcov-ignore

        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[
                &ObjectKinematicsUpdateSystem::type_name(),
                &KeepWithinMapBoundsSystem::type_name(),
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

        // Reduces charge when not charging.
        builder.add(
            ChargeRetentionSystem::new(),
            &ChargeRetentionSystem::type_name(),
            &[],
        ); // kcov-ignore

        // Reduces `StunPoints` each tick.
        builder.add(
            StunPointsReductionSystem::new(),
            &StunPointsReductionSystem::type_name(),
            &[],
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
            &[
                &StunPointsReductionSystem::type_name(),
                &HitRepeatTrackersTickerSystem::type_name(),
            ],
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
            SequenceEndTransitionSystem::new(),
            &SequenceEndTransitionSystem::type_name(),
            &[],
        ); // kcov-ignore

        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        //
        // It also depends on `&SequenceEndTransitionSystem` as the
        // `CharacterSequenceUpdater` transitions should overwrite the `SequenceEndTransition`
        // update.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[&SequenceEndTransitionSystem::type_name()],
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

        // Charging
        builder.add(
            ChargeInitializeDetectionSystem::new(),
            &ChargeInitializeDetectionSystem::type_name(),
            &[&CharacterControlTransitionsTransitionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ChargeInitializeDelaySystem::new(),
            &ChargeInitializeDelaySystem::type_name(),
            &[&ChargeInitializeDetectionSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ChargeIncrementSystem::new(),
            &ChargeIncrementSystem::type_name(),
            &[&ChargeInitializeDelaySystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ChargeUsageSystem::new(),
            &ChargeUsageSystem::type_name(),
            &[&ChargeIncrementSystem::type_name()],
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
        builder.add(
            CpBarUpdateSystem::new(),
            &CpBarUpdateSystem::type_name(),
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

        builder.add(
            CameraTrackingSystem::default(),
            &CameraTrackingSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CameraVelocitySystem::default(),
            &CameraVelocitySystem::type_name(),
            &[&CameraTrackingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CameraOrthoSystem::default(),
            "camera_ortho",
            &[&CameraTrackingSystem::type_name()],
        ); // kcov-ignore

        let position_tracker_system =
            LastTrackerSystem::<Position<f32>>::new(stringify!(Position<f32>));
        let position_tracker_system_name = position_tracker_system.system_name();
        builder.add(position_tracker_system, &position_tracker_system_name, &[]); // kcov-ignore

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

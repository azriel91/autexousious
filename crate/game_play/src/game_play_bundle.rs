use std::any;

use amethyst::{
    core::{bundle::SystemBundle, SystemExt},
    ecs::{DispatcherBuilder, World},
    Error,
};
use audio_model::loaded::{SourceSequence, SourceSequenceHandles};
use audio_play::SequenceAudioPlaySystem;
use camera_play::{CameraTrackingSystem, CameraVelocitySystem};
use character_model::{
    config::CharacterIrr,
    loaded::{CharacterIrs, CharacterIrsHandles},
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
    CollisionDetectionSystem, ContactDetectionSystem, HitDetectionSystem, HitEffectSystem,
    HitRepeatTrackersAugmentSystem, HitRepeatTrackersTickerSystem, HittingEffectSystem,
};
use derive_new::new;
use game_input::ControllerInput;
use game_play_hud::{CpBarUpdateSystem, HpBarUpdateSystem};
use input_reaction_model::{
    config::BasicIrr,
    loaded::{InputReactionsSequence, InputReactionsSequenceHandles},
};
use input_reaction_play::{ButtonInputReactionsTransitionSystem, InputReactionsTransitionSystem};
use kinematic_model::{
    config::Position,
    loaded::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandles},
};
use map_play::{
    KeepWithinMapBoundsSystem, MapEnterExitDetectionSystem, MapOutOfBoundsClockAugmentSystem,
    MapOutOfBoundsDeletionSystem, MapSpawnOutOfBoundsDetectionSystem,
};
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
use sprite_model::loaded::{
    ScaleSequence, ScaleSequenceHandles, SpriteRenderSequence, SpriteRenderSequenceHandles,
    TintSequence, TintSequenceHandles,
};
use sprite_play::SpriteScaleUpdateSystem;
use state_registry::StateId;
use tracker::LastTrackerSystem;

use crate::{
    CharacterHitEffectSystem, CharacterSequenceUpdateSystem, FrameFreezeClockAugmentSystem,
    GamePlayEndDetectionSystem, GamePlayEndTransitionDelaySystem, GamePlayEndTransitionSystem,
    GamePlayRemovalAugmentSystem, GamePlayStatusDisplaySystem, GroundingFrictionSystem,
    ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
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
            any::type_name::<SequenceStatusUpdateSystem>(),
            &[],
        ); // kcov-ignore

        macro_rules! sequence_component_update_system {
            ($component_data_type:path) => {
                let system_name = format!(
                    "{}{}",
                    any::type_name::<SequenceComponentUpdateSystem::<$component_data_type>>(),
                    concat!("<", stringify!($component_data_type), ">")
                );
                builder.add(
                    SequenceComponentUpdateSystem::<$component_data_type>::new(),
                    &system_name,
                    &[any::type_name::<SequenceStatusUpdateSystem>()],
                ); // kcov-ignore
            };
        }

        sequence_component_update_system!(WaitSequenceHandles);
        sequence_component_update_system!(SourceSequenceHandles);
        sequence_component_update_system!(ObjectAccelerationSequenceHandles);
        sequence_component_update_system!(SpriteRenderSequenceHandles);
        sequence_component_update_system!(BodySequenceHandles);
        sequence_component_update_system!(InteractionsSequenceHandles);
        sequence_component_update_system!(SpawnsSequenceHandles);
        sequence_component_update_system!(SequenceEndTransitions);
        sequence_component_update_system!(TintSequenceHandles);
        sequence_component_update_system!(ScaleSequenceHandles);
        sequence_component_update_system!(CharacterIrsHandles);
        sequence_component_update_system!(InputReactionsSequenceHandles);

        // TODO: The `SequenceUpdateSystem`s depend on the following systems:
        //
        // * `SequenceComponentUpdateSystem::<_, _>`
        //
        // Because there are so many, and we haven't implemented a good way to specify the
        // dependencies without heaps of duplicated code, we use a barrier.
        //
        // TODO: We can potentially use the `inventory` or `linkme` crates to generate the systems
        // and dependencies.
        builder.add_barrier();

        // Updates frame limit and ticks the sequence logic clocks.
        builder.add(
            SequenceUpdateSystem::new(),
            any::type_name::<SequenceUpdateSystem>(),
            &[
                // any::type_name::<SequenceComponentUpdateSystem::<_, _>>(),
            ],
        ); // kcov-ignore

        macro_rules! frame_component_update_system {
            ($frame_component_data:ident) => {
                builder.add(
                    FrameComponentUpdateSystem::<$frame_component_data>::new(),
                    any::type_name::<FrameComponentUpdateSystem<$frame_component_data>>(),
                    &[any::type_name::<SequenceUpdateSystem>()],
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
        frame_component_update_system!(TintSequence);
        frame_component_update_system!(ScaleSequence);
        frame_component_update_system!(CharacterIrs);
        frame_component_update_system!(InputReactionsSequence);

        builder.add(
            FrameFreezeClockAugmentSystem::new(),
            any::type_name::<FrameFreezeClockAugmentSystem>(),
            &[any::type_name::<SequenceUpdateSystem>()],
        ); // kcov-ignore
        builder.add(
            HitRepeatTrackersAugmentSystem::new(),
            any::type_name::<HitRepeatTrackersAugmentSystem>(),
            &[],
        ); // kcov-ignore

        builder.add(HitSfxSystem::new(), any::type_name::<HitSfxSystem>(), &[]);

        // Play sounds from sequence updates.
        builder.add(
            SequenceAudioPlaySystem::new(),
            any::type_name::<SequenceAudioPlaySystem>(),
            &[any::type_name::<FrameComponentUpdateSystem<SourceSequence>>()],
        ); // kcov-ignore

        // Spawn objects
        builder.add(
            SpawnGameObjectSystem::new(),
            any::type_name::<SpawnGameObjectSystem>(),
            &[any::type_name::<FrameComponentUpdateSystem<SpawnsSequence>>()],
        ); // kcov-ignore
        builder.add(
            SpawnGameObjectRectifySystem::new(),
            any::type_name::<SpawnGameObjectRectifySystem>(),
            &[any::type_name::<SpawnGameObjectSystem>()],
        ); // kcov-ignore
        builder.add(
            MapSpawnOutOfBoundsDetectionSystem::new().pausable(StateId::GamePlay),
            any::type_name::<MapSpawnOutOfBoundsDetectionSystem>(),
            &[any::type_name::<SpawnGameObjectRectifySystem>()],
        ); // kcov-ignore
        builder.add(
            GamePlayRemovalAugmentSystem::new(),
            any::type_name::<GamePlayRemovalAugmentSystem>(),
            &[any::type_name::<SpawnGameObjectSystem>()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Component value update === //

        // transform.scale_mut().{x/y/z} = `Scale`
        builder.add(
            SpriteScaleUpdateSystem::new(),
            any::type_name::<SpriteScaleUpdateSystem>(),
            &[],
        ); // kcov-ignore

        // vel += `ObjectAcceleration` (from frame config).
        builder.add(
            ObjectAccelerationSystem::new(),
            any::type_name::<ObjectAccelerationSystem>(),
            &[],
        ); // kcov-ignore

        // pos += vel
        // This must be between the `FrameFreezeClockAugmentSystem` and `SequenceUpdateSystem`s
        // since it needs to wait for the `FrameFreezeClock` to tick.
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            any::type_name::<ObjectKinematicsUpdateSystem>(),
            &[any::type_name::<ObjectAccelerationSystem>()],
        ); // kcov-ignore

        // `Position` correction based on margins.
        // vel += mass
        builder.add(
            ObjectGravitySystem::new(),
            any::type_name::<ObjectGravitySystem>(),
            &[any::type_name::<ObjectKinematicsUpdateSystem>()],
        ); // kcov-ignore
        builder.add(
            MapEnterExitDetectionSystem::new().pausable(StateId::GamePlay),
            any::type_name::<MapEnterExitDetectionSystem>(),
            &[any::type_name::<ObjectGravitySystem>()],
        ); // kcov-ignore
        builder.add(
            KeepWithinMapBoundsSystem::new().pausable(StateId::GamePlay),
            any::type_name::<KeepWithinMapBoundsSystem>(),
            &[any::type_name::<MapEnterExitDetectionSystem>()],
        ); // kcov-ignore
        builder.add(
            ObjectGroundingSystem::new().pausable(StateId::GamePlay),
            any::type_name::<ObjectGroundingSystem>(),
            &[any::type_name::<MapEnterExitDetectionSystem>()],
        ); // kcov-ignore

        // Updates `Velocity<f32>` based on grounding.
        builder.add(
            GroundingFrictionSystem::new(),
            any::type_name::<GroundingFrictionSystem>(),
            &[any::type_name::<ObjectGroundingSystem>()],
        ); // kcov-ignore

        builder.add(
            MapOutOfBoundsDeletionSystem::new(),
            any::type_name::<MapOutOfBoundsDeletionSystem>(),
            &[
                any::type_name::<MapEnterExitDetectionSystem>(),
                any::type_name::<MapSpawnOutOfBoundsDetectionSystem>(),
            ],
        ); // kcov-ignore
        builder.add(
            MapOutOfBoundsClockAugmentSystem::new(),
            any::type_name::<MapOutOfBoundsClockAugmentSystem>(),
            &[any::type_name::<MapOutOfBoundsDeletionSystem>()],
        ); // kcov-ignore

        builder.add(
            ObjectTransformUpdateSystem::new(),
            any::type_name::<ObjectTransformUpdateSystem>(),
            &[
                any::type_name::<ObjectKinematicsUpdateSystem>(),
                any::type_name::<KeepWithinMapBoundsSystem>(),
            ],
        ); // kcov-ignore
        builder.add(
            ObjectMirroringSystem::new(),
            any::type_name::<ObjectMirroringSystem>(),
            &[any::type_name::<ObjectTransformUpdateSystem>()],
        ); // kcov-ignore
        builder.add(
            StickToTargetObjectSystem::new(),
            any::type_name::<StickToTargetObjectSystem>(),
            &[any::type_name::<ObjectTransformUpdateSystem>()],
        ); // kcov-ignore

        // Reduces charge when not charging.
        builder.add(
            ChargeRetentionSystem::new(),
            any::type_name::<ChargeRetentionSystem>(),
            &[],
        ); // kcov-ignore

        // Reduces `StunPoints` each tick.
        builder.add(
            StunPointsReductionSystem::new(),
            any::type_name::<StunPointsReductionSystem>(),
            &[],
        ); // kcov-ignore

        builder.add(
            HitRepeatTrackersTickerSystem::new(),
            any::type_name::<HitRepeatTrackersTickerSystem>(),
            &[any::type_name::<HitRepeatTrackersAugmentSystem>()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Effect Detection === //

        builder.add(
            CollisionDetectionSystem::new(),
            any::type_name::<CollisionDetectionSystem>(),
            &[
                any::type_name::<StunPointsReductionSystem>(),
                any::type_name::<HitRepeatTrackersTickerSystem>(),
            ],
        ); // kcov-ignore
        builder.add(
            ContactDetectionSystem::new(),
            any::type_name::<ContactDetectionSystem>(),
            &[any::type_name::<CollisionDetectionSystem>()],
        ); // kcov-ignore
        builder.add(
            HitDetectionSystem::new(),
            any::type_name::<HitDetectionSystem>(),
            &[any::type_name::<ContactDetectionSystem>()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Sequence ID Updates === //

        builder.add(
            SequenceEndTransitionSystem::new(),
            any::type_name::<SequenceEndTransitionSystem>(),
            &[],
        ); // kcov-ignore

        builder.add(
            InputReactionsTransitionSystem::<BasicIrr>::new(),
            &any::type_name::<InputReactionsTransitionSystem<BasicIrr>>(),
            &[any::type_name::<SequenceEndTransitionSystem>()],
        ); // kcov-ignore
        builder.add(
            ButtonInputReactionsTransitionSystem::<BasicIrr>::new(),
            &any::type_name::<ButtonInputReactionsTransitionSystem<BasicIrr>>(),
            &[any::type_name::<SequenceEndTransitionSystem>()],
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
            any::type_name::<CharacterSequenceUpdateSystem>(),
            &[any::type_name::<SequenceEndTransitionSystem>()],
        ); // kcov-ignore
        builder.add(
            InputReactionsTransitionSystem::<CharacterIrr>::new(),
            &any::type_name::<InputReactionsTransitionSystem<CharacterIrr>>(),
            &[any::type_name::<CharacterSequenceUpdateSystem>()],
        ); // kcov-ignore
        builder.add(
            CharacterHitEffectSystem::new(),
            any::type_name::<CharacterHitEffectSystem>(),
            &[&any::type_name::<
                InputReactionsTransitionSystem<CharacterIrr>,
            >()],
        ); // kcov-ignore

        // Charging
        builder.add(
            ChargeInitializeDetectionSystem::new(),
            any::type_name::<ChargeInitializeDetectionSystem>(),
            &[&any::type_name::<
                InputReactionsTransitionSystem<CharacterIrr>,
            >()],
        ); // kcov-ignore
        builder.add(
            ChargeInitializeDelaySystem::new(),
            any::type_name::<ChargeInitializeDelaySystem>(),
            &[any::type_name::<ChargeInitializeDetectionSystem>()],
        ); // kcov-ignore
        builder.add(
            ChargeIncrementSystem::new(),
            any::type_name::<ChargeIncrementSystem>(),
            &[any::type_name::<ChargeInitializeDelaySystem>()],
        ); // kcov-ignore
        builder.add(
            ChargeUsageSystem::new(),
            any::type_name::<ChargeUsageSystem>(),
            &[any::type_name::<ChargeIncrementSystem>()],
        ); // kcov-ignore

        // Hit / Hitting effects.
        //
        // There are only two currently, but if there is a timer system, perhaps that should go
        // last.
        // The `HitEffectSystem` depends on the `HittingEffectSystem` to ensure the
        // `Hit` sequence is deterministic and overwrites the `Hitting` sequence.
        builder.add(
            HittingEffectSystem::new(),
            any::type_name::<HittingEffectSystem>(),
            &[],
        ); // kcov-ignore
        builder.add(
            HitEffectSystem::new(),
            any::type_name::<HitEffectSystem>(),
            &[any::type_name::<HittingEffectSystem>()],
        ); // kcov-ignore

        // Perhaps this should be straight after the `StickToTargetObjectSystem`, but we put it here
        // so that the renderer will show the HP including the damage dealt this frame, instead of
        // one frame later.
        builder.add(
            HpBarUpdateSystem::new(),
            any::type_name::<HpBarUpdateSystem>(),
            &[any::type_name::<CharacterHitEffectSystem>()],
        ); // kcov-ignore
        builder.add(
            CpBarUpdateSystem::new(),
            any::type_name::<CpBarUpdateSystem>(),
            &[any::type_name::<CharacterHitEffectSystem>()],
        ); // kcov-ignore

        builder.add_barrier();

        // === Helper Systems === //

        // Detects when the winning condition has been met.
        builder.add(
            GamePlayEndDetectionSystem::new(),
            any::type_name::<GamePlayEndDetectionSystem>(),
            &[],
        ); // kcov-ignore

        builder.add(
            GamePlayStatusDisplaySystem::new(),
            any::type_name::<GamePlayStatusDisplaySystem>(),
            &[any::type_name::<GamePlayEndDetectionSystem>()],
        ); // kcov-ignore

        // Delay before game play end transition is accepted.
        builder.add(
            GamePlayEndTransitionDelaySystem::new(),
            any::type_name::<GamePlayEndTransitionDelaySystem>(),
            &[any::type_name::<GamePlayEndDetectionSystem>()],
        ); // kcov-ignore

        // Sends a state transition when game play ends, and `Attack` is pressed.
        builder.add(
            GamePlayEndTransitionSystem::new(),
            any::type_name::<GamePlayEndTransitionSystem>(),
            &[any::type_name::<GamePlayEndTransitionDelaySystem>()],
        ); // kcov-ignore

        builder.add(
            CameraTrackingSystem::default().pausable(StateId::GamePlay),
            any::type_name::<CameraTrackingSystem>(),
            &[],
        ); // kcov-ignore
        builder.add(
            CameraVelocitySystem::default(),
            any::type_name::<CameraVelocitySystem>(),
            &[any::type_name::<CameraTrackingSystem>()],
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
            &[any::type_name::<GamePlayEndTransitionSystem>()],
        ); // kcov-ignore

        Ok(())
    }
}

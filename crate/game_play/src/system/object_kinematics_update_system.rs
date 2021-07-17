use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use sequence_model::play::FrameFreezeClock;

/// Updates each entity's `Position` based on their `Velocity` in game.
///
/// This system should be run after all other systems that affect kinematics
/// have run.
#[derive(Debug, Default, new)]
pub struct ObjectKinematicsUpdateSystem;

/// `ObjectKinematicsUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectKinematicsUpdateSystemData<'s> {
    /// `FrameFreezeClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_freeze_clocks: ReadStorage<'s, FrameFreezeClock>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: ReadStorage<'s, Velocity<f32>>,
}

impl<'s> System<'s> for ObjectKinematicsUpdateSystem {
    type SystemData = ObjectKinematicsUpdateSystemData<'s>;

    fn run(
        &mut self,
        ObjectKinematicsUpdateSystemData {
            frame_freeze_clocks,
            mut positions,
            velocities,
        }: Self::SystemData,
    ) {
        (frame_freeze_clocks.maybe(), &mut positions, &velocities)
            .join()
            .for_each(|(frame_freeze_clock, position, velocity)| {
                let frozen = frame_freeze_clock
                    .map(|frame_freeze_clock| !frame_freeze_clock.is_complete())
                    .unwrap_or(false);
                if !frozen {
                    position.0 += velocity.0;
                }
            })
    }
}

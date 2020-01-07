use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Velocity;
use object_model::play::Grounding;

/// Updates `Velocity<f32>` based on grounding.
#[derive(Debug, Default, new)]
pub struct GroundingFrictionSystem;

/// `GroundingFrictionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GroundingFrictionSystemData<'s> {
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: ReadStorage<'s, Grounding>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl<'s> System<'s> for GroundingFrictionSystem {
    type SystemData = GroundingFrictionSystemData<'s>;

    fn run(
        &mut self,
        GroundingFrictionSystemData {
            groundings,
            mut velocities,
        }: Self::SystemData,
    ) {
        (&groundings, &mut velocities)
            .join()
            .for_each(|(grounding, velocity)| match grounding {
                Grounding::OnGround => {
                    if velocity[0].abs() < 11. {
                        velocity[0] = 0.;
                    } else {
                        velocity[0] /= 2.;
                    }

                    velocity[1] = 0.;

                    if velocity[2].abs() < 7. {
                        velocity[2] = 0.;
                    } else {
                        velocity[2] /= 2.;
                    }
                }
                Grounding::Airborne | Grounding::Underground => {}
            });
    }
}

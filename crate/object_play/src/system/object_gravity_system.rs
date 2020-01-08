use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Velocity;
use object_model::{config::Mass, play::Grounding};

/// Increases velocity of `Object`s that have `Mass` and are `Airborne`.
#[derive(Debug, Default, new)]
pub struct ObjectGravitySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectGravitySystemData<'s> {
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: ReadStorage<'s, Grounding>,
    /// `Mass` components.
    #[derivative(Debug = "ignore")]
    pub masses: ReadStorage<'s, Mass>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl<'s> System<'s> for ObjectGravitySystem {
    type SystemData = ObjectGravitySystemData<'s>;

    fn run(
        &mut self,
        ObjectGravitySystemData {
            groundings,
            masses,
            mut velocities,
        }: Self::SystemData,
    ) {
        (&groundings, &masses, &mut velocities)
            .join()
            .filter_map(|(grounding, mass, velocity)| {
                if *grounding == Grounding::Airborne {
                    Some((mass, velocity))
                } else {
                    None
                }
            })
            .for_each(|(mass, velocity)| {
                velocity[1] -= **mass; // No gravity yet, so we just use `Mass` as weight.
            });
    }
}

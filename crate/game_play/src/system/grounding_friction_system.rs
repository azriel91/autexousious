use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Velocity;
use object_model::play::Grounding;

use typename_derive::TypeName;

/// Updates `Velocity<f32>` based on grounding.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct GroundingFrictionSystem;

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
                Grounding::Airborne => {}
            });
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Velocity;
    use object_model::play::Grounding;
    use typename::TypeName;

    use super::GroundingFrictionSystem;

    #[test]
    fn divides_x_velocity_by_two_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                velocity: Velocity::new(11., 0., 0.),
            },
            Velocity::new(5.5, 0., 0.),
        )
    }

    #[test]
    fn divides_z_velocity_by_two_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                velocity: Velocity::new(0., 0., 7.),
            },
            Velocity::new(0., 0., 3.5),
        )
    }

    #[test]
    fn zeroes_y_velocity_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                velocity: Velocity::new(0., -15., 0.),
            },
            Velocity::new(0., 0., 0.),
        )
    }

    #[test]
    fn zeroes_x_velocity_when_less_than_10_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                velocity: Velocity::new(9.99, 0., 0.),
            },
            Velocity::new(0., 0., 0.),
        )
    }

    #[test]
    fn zeroes_z_velocity_when_less_than_7_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                velocity: Velocity::new(0., 0., 6.99),
            },
            Velocity::new(0., 0., 0.),
        )
    }

    fn run_test(
        SetupParams {
            grounding,
            velocity: velocity_setup,
            ..
        }: SetupParams,
        velocity_expected: Velocity<f32>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            // kcov-ignore-start
            .with_system(
                GroundingFrictionSystem::new(),
                GroundingFrictionSystem::type_name(),
                &[],
            )
            // kcov-ignore-end
            .with_setup(move |world| {
                let entity = world
                    .create_entity()
                    .with(grounding)
                    .with(velocity_setup)
                    .build();
                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let velocities = world.read_storage::<Velocity<f32>>();
                let velocity_actual = velocities
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `Velocity<f32>` component.");

                assert_eq!(velocity_expected, velocity_actual);
            })
            .run()
    }

    #[derive(Debug)]
    struct SetupParams {
        grounding: Grounding,
        velocity: Velocity<f32>,
    }
}

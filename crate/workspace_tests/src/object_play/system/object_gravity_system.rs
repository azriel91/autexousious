#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Velocity;
    use object_model::{config::Mass, play::Grounding};
    use typename::TypeName;

    use object_play::ObjectGravitySystem;

    #[test]
    fn decreases_velocity_by_mass_when_airborne() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::Airborne,
                mass: Mass::new(2.),
                velocity: Velocity::new(0., 10., 0.),
            },
            ExpectedParams {
                velocity: Velocity::new(0., 8., 0.),
            },
        )
    }

    #[test]
    fn no_change_to_velocity_when_on_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                mass: Mass::new(2.),
                velocity: Velocity::new(0., 10., 0.),
            },
            ExpectedParams {
                velocity: Velocity::new(0., 10., 0.),
            },
        )
    }

    fn run_test(
        SetupParams {
            grounding,
            mass,
            velocity: velocity_setup,
        }: SetupParams,
        ExpectedParams {
            velocity: velocity_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                ObjectGravitySystem::new(),
                ObjectGravitySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(grounding)
                    .with(mass)
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

    struct SetupParams {
        grounding: Grounding,
        mass: Mass,
        velocity: Velocity<f32>,
    }

    struct ExpectedParams {
        velocity: Velocity<f32>,
    }
}

use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
};
use camera_model::play::CameraTargetCoordinates;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use typename_derive::TypeName;

/// Updates camera velocity to smoothen camera movement between its current and target position.
#[derive(Debug, Default, TypeName, new)]
pub struct CameraVelocitySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CameraVelocitySystemData<'s> {
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: ReadStorage<'s, Camera>,
    /// `CameraTargetCoordinates` components.
    #[derivative(Debug = "ignore")]
    pub camera_target_coordinateses: ReadStorage<'s, CameraTargetCoordinates>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl<'s> System<'s> for CameraVelocitySystem {
    type SystemData = CameraVelocitySystemData<'s>;

    fn run(
        &mut self,
        CameraVelocitySystemData {
            cameras,
            camera_target_coordinateses,
            positions,
            mut velocities,
        }: Self::SystemData,
    ) {
        (
            &cameras,
            &camera_target_coordinateses,
            &positions,
            &mut velocities,
        )
            .join()
            .for_each(|(_, camera_target_coordinates, position, velocity)| {
                **velocity = {
                    // 1. Get distance between current position and target position.
                    //    Divide that by 10, this is the max velocity we will reach.
                    //
                    //     e.g. if we have to move 1000 pixels, at most we will move 100 per tick.
                    //
                    // 2. Calculate an average between the current velocity and the target velocity.
                    //
                    //     If our current velocity is 0, then we will increase to 50.
                    //     Next frame will be 75: (50 + 100) / 2
                    //
                    //     If our current velocity is 200, then we will decrease to 150.
                    //     Next frame will be 125: (150 + 100) / 2
                    //
                    let velocity_limit = (**camera_target_coordinates - **position) / 10.;
                    (**velocity + velocity_limit) / 2.
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Entity, WorldExt, WriteStorage},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use camera_model::play::CameraTargetCoordinates;
    use kinematic_model::config::{Position, Velocity};
    use pretty_assertions::assert_eq;
    use typename::TypeName;

    use super::CameraVelocitySystem;
    use crate::CameraCreator;

    #[test]
    fn velocity_remains_zero_when_position_matches_target() -> Result<(), Error> {
        run_test(
            SetupParams {
                camera_target_coordinates: CameraTargetCoordinates::new(100., 200., 300.),
                position: Position::new(100., 200., 300.),
                velocity: Velocity::new(0., 0., 0.),
            },
            ExpectedParams {
                velocity_steps: vec![Velocity::new(0., 0., 0.), Velocity::new(0., 0., 0.)],
            },
        )
    }

    #[test]
    fn velocity_increments_smoothly_when_target_is_far() -> Result<(), Error> {
        run_test(
            SetupParams {
                camera_target_coordinates: CameraTargetCoordinates::new(100., 200., 300.),
                position: Position::new(0., 0., 0.),
                velocity: Velocity::new(0., 0., 0.),
            },
            ExpectedParams {
                velocity_steps: vec![
                    Velocity::new(5., 10., 15.),
                    Velocity::new(7.5, 15., 22.5),
                    Velocity::new(8.75, 17.5, 26.25),
                ],
            },
        )
    }

    fn run_test(
        SetupParams {
            camera_target_coordinates,
            position,
            velocity: velocity_setup,
        }: SetupParams,
        ExpectedParams { velocity_steps }: ExpectedParams,
    ) -> Result<(), Error> {
        let mut amethyst_application = AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_system(
                CameraVelocitySystem::new(),
                CameraVelocitySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let camera_entity = CameraCreator::create_in_world(world);

                {
                    let (mut camera_target_coordinateses, mut positions, mut velocities) = world
                        .system_data::<(
                            WriteStorage<'_, CameraTargetCoordinates>,
                            WriteStorage<'_, Position<f32>>,
                            WriteStorage<'_, Velocity<f32>>,
                        )>();

                    camera_target_coordinateses
                        .insert(camera_entity, camera_target_coordinates)
                        .expect("Failed to insert `CameraTargetCoordinates` component.");
                    positions
                        .insert(camera_entity, position)
                        .expect("Failed to insert `Position<f32>` component.");
                    velocities
                        .insert(camera_entity, velocity_setup)
                        .expect("Failed to insert `Velocity<f32>` component.");
                }

                world.insert(camera_entity);
            });

        amethyst_application = velocity_steps.into_iter().fold(
            amethyst_application,
            |amethyst_application, velocity_expected| {
                amethyst_application.with_assertion(move |world| {
                    let entity = *world.read_resource::<Entity>();
                    let velocities = world.read_storage::<Velocity<f32>>();
                    let velocity_actual = velocities
                        .get(entity)
                        .copied()
                        .expect("Expected entity to have `Transform` component.");

                    assert_eq!(velocity_expected, velocity_actual);
                })
            },
        );

        amethyst_application.run()
    }

    struct SetupParams {
        camera_target_coordinates: CameraTargetCoordinates,
        position: Position<f32>,
        velocity: Velocity<f32>,
    }

    struct ExpectedParams {
        velocity_steps: Vec<Velocity<f32>>,
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, WorldExt},
        input::StringBindings,
        renderer::camera::Camera,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Position;
    use typename::TypeName;

    use game_play::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                camera: None,
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -11., 1.)),
            },
        )
    }

    #[test]
    fn updates_camera_transform_xyz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                camera: Some(Camera::standard_2d(800., 600.)),
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -10., 1.)),
            },
        )
    }

    fn run_test(
        SetupParams { position, camera }: SetupParams,
        ExpectedParams {
            transform: transform_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_system(
                ObjectTransformUpdateSystem::new(),
                ObjectTransformUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(position)
                        .with(Transform::default());

                    if let Some(camera) = camera.clone() {
                        entity_builder = entity_builder.with(camera);
                    }

                    entity_builder.build()
                };

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let transforms = world.read_storage::<Transform>();

                let transform_actual = transforms
                    .get(entity)
                    .expect("Expected entity to have `Transform` component.");
                assert_eq!(
                    transform_expected.translation(),
                    transform_actual.translation()
                );
            })
            .run()
    }

    struct SetupParams {
        position: Position<f32>,
        camera: Option<Camera>,
    }
    struct ExpectedParams {
        transform: Transform,
    }
}

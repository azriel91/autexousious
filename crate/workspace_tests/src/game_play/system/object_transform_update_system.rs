#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, WorldExt},
        input::StringBindings,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::{config::Position, play::PositionZAsY};
    use std::any;

    use game_play::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz_when_position_z_as_y_present() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                position_z_as_y: true,
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -11., 1.)),
            },
        )
    }

    #[test]
    fn updates_transform_xyz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                position_z_as_y: false,
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -10., 1.)),
            },
        )
    }

    fn run_test(
        SetupParams {
            position,
            position_z_as_y,
        }: SetupParams,
        ExpectedParams {
            transform: transform_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_system(
                ObjectTransformUpdateSystem::new(),
                any::type_name::<ObjectTransformUpdateSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(position)
                        .with(Transform::default());

                    if position_z_as_y {
                        entity_builder = entity_builder.with(PositionZAsY);
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
        position_z_as_y: bool,
    }
    struct ExpectedParams {
        transform: Transform,
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{
            math::Vector3,
            transform::{Transform, TransformBundle},
        },
        ecs::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use sprite_model::config::Scale;
    use typename::TypeName;

    use sprite_play::SpriteScaleUpdateSystem;

    #[test]
    fn updates_transform_scale() -> Result<(), Error> {
        let mut transform_setup = Transform::default();
        transform_setup.set_scale(Vector3::new(3.5, 3.5, 3.5));

        let mut transform_expected = Transform::default();
        transform_expected.set_scale(Vector3::new(2.1, 2.1, 2.1));

        run_test(
            SetupParams {
                scale: Scale::new(2.1),
                transform: transform_setup,
            },
            ExpectedParams {
                transform: transform_expected,
            },
        )
    }

    fn run_test(
        SetupParams {
            scale,
            transform: transform_setup,
        }: SetupParams,
        ExpectedParams {
            transform: transform_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_system(
                SpriteScaleUpdateSystem::new(),
                SpriteScaleUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(scale)
                    .with(transform_setup)
                    .build();

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let transforms = world.read_storage::<Transform>();

                let transform_actual = transforms
                    .get(entity)
                    .cloned()
                    .expect("Expected entity to have `Transform` component.");
                assert_eq!(transform_expected.scale(), transform_actual.scale());
            })
            .run()
    }

    struct SetupParams {
        scale: Scale,
        transform: Transform,
    }

    struct ExpectedParams {
        transform: Transform,
    }
}

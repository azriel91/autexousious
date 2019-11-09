#[cfg(test)]
mod test {
    use amethyst::{
        core::transform::TransformBundle,
        ecs::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Position;
    use sprite_model::config::SpritePosition;
    use typename::TypeName;

    use sprite_play::SpritePositionUpdateSystem;

    #[test]
    fn updates_position_xyz() -> Result<(), Error> {
        run_test(
            SetupParams {
                sprite_position: SpritePosition::new(100, -10, 1),
            },
            ExpectedParams {
                position: Position::new(100., -10., 1.),
            },
        )
    }

    fn run_test(
        SetupParams { sprite_position }: SetupParams,
        ExpectedParams {
            position: position_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_system(
                SpritePositionUpdateSystem::new(),
                SpritePositionUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = world.create_entity().with(sprite_position).build();

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let positions = world.read_storage::<Position<f32>>();

                let position_actual = positions
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `Position<f32>` component.");
                assert_eq!(position_expected, position_actual);
            })
            .run()
    }

    struct SetupParams {
        sprite_position: SpritePosition,
    }

    struct ExpectedParams {
        position: Position<f32>,
    }
}

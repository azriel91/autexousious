#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        core::{math::RealField, transform::Transform},
        ecs::{Join, ReadStorage, WriteStorage},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use approx::assert_relative_eq;
    use mirrored_model::play::Mirrored;

    use object_play::ObjectMirroringSystem;

    #[test]
    fn rotates_mirrored_objects_around_y_axis() -> Result<(), Error> {
        run_test(
            |(mirrored, _transform)| **mirrored = true,
            |(_mirrored, transform)| assert_relative_eq!(f32::pi(), transform.rotation().angle()),
        )
    }

    #[test]
    fn resets_non_mirrored_objects_y_axis_rotation() -> Result<(), Error> {
        run_test(
            |(mirrored, transform)| {
                **mirrored = false;
                transform.set_rotation_y_axis(f32::pi());
            },
            |(_mirrored, transform)| assert_relative_eq!(0., transform.rotation().angle()),
        )
    }

    fn run_test(
        setup_fn: fn((&mut Mirrored, &mut Transform)),
        assertion_fn: fn((&Mirrored, &Transform)),
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(move |world| {
                let (mut mirroreds, mut transforms) = world
                    .system_data::<(WriteStorage<'_, Mirrored>, WriteStorage<'_, Transform>)>();
                (&mut mirroreds, &mut transforms).join().for_each(setup_fn)
            })
            .with_system_single(
                ObjectMirroringSystem::new(),
                any::type_name::<ObjectMirroringSystem>(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let (mirroreds, transforms) =
                    world.system_data::<(ReadStorage<'_, Mirrored>, ReadStorage<'_, Transform>)>();
                (&mirroreds, &transforms).join().for_each(assertion_fn)
            })
            .run_winit_loop()
    }
}

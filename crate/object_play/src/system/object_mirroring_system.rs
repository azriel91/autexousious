use amethyst::{
    core::{math::RealField, transform::Transform},
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use object_model::play::Mirrored;
use typename_derive::TypeName;

/// Rotates `Transform` (and hence, sprites) of `Object`s that are `Mirrored`.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectMirroringSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectMirroringSystemData<'s> {
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for ObjectMirroringSystem {
    type SystemData = ObjectMirroringSystemData<'s>;

    fn run(
        &mut self,
        ObjectMirroringSystemData {
            mirroreds,
            mut transforms,
        }: Self::SystemData,
    ) {
        (&mirroreds, &mut transforms)
            .join()
            .for_each(|(mirrored, transform)| {
                if mirrored.0 {
                    transform.set_rotation_y_axis(f32::pi());
                } else {
                    transform.set_rotation_y_axis(0.);
                };
            });
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::RealField, transform::Transform},
        ecs::{Join, ReadStorage, WriteStorage},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use approx::assert_relative_eq;
    use object_model::play::Mirrored;
    use typename::TypeName;

    use super::ObjectMirroringSystem;

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
                ObjectMirroringSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let (mirroreds, transforms) =
                    world.system_data::<(ReadStorage<'_, Mirrored>, ReadStorage<'_, Transform>)>();
                (&mirroreds, &transforms).join().for_each(assertion_fn)
            })
            .run_isolated()
    }
}

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

use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use sprite_model::config::Scale;
use typename_derive::TypeName;

/// Updates the `Transform`'s scaling of entities that have a `Scale` component.
#[derive(Debug, Default, TypeName, new)]
pub struct SpriteScaleUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpriteScaleUpdateSystemData<'s> {
    /// `Scale` components.
    #[derivative(Debug = "ignore")]
    pub scales: ReadStorage<'s, Scale>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for SpriteScaleUpdateSystem {
    type SystemData = SpriteScaleUpdateSystemData<'s>;

    fn run(
        &mut self,
        SpriteScaleUpdateSystemData {
            scales,
            mut transforms,
        }: Self::SystemData,
    ) {
        (&scales, &mut transforms)
            .join()
            .for_each(|(scale, transform)| {
                let scale = **scale;
                let transform_scale = transform.scale_mut();
                transform_scale.x = scale;
                transform_scale.y = scale;
                transform_scale.z = scale;
            });
    } // kcov-ignore
}

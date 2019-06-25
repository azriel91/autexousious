use amethyst::{
    core::{Float, Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use derivative::Derivative;
use derive_new::new;
use object_model::play::{HealthPoints, ParentObject};
use shred_derive::SystemData;
use typename_derive::TypeName;

use crate::{HpBar, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT};

/// Move HpBar below character.
const Y_OFFSET: f32 = -10.;
/// Move HpBar in front of object.
const Z_OFFSET: f32 = 1.;

/// Updates `HpBar` length based on its parent entity's `HealthPoints`.
#[derive(Debug, Default, TypeName, new)]
pub struct HpBarUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct HpBarUpdateSystemData<'s> {
    /// `HpBar` components.
    #[derivative(Debug = "ignore")]
    pub hp_bars: ReadStorage<'s, HpBar>,
    /// `ParentObject` components.
    #[derivative(Debug = "ignore")]
    pub parent_objects: ReadStorage<'s, ParentObject>,
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: ReadStorage<'s, HealthPoints>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
}

impl<'s> System<'s> for HpBarUpdateSystem {
    type SystemData = HpBarUpdateSystemData<'s>;

    fn run(
        &mut self,
        HpBarUpdateSystemData {
            hp_bars,
            parent_objects,
            health_pointses,
            mut transforms,
            mut sprite_renders,
        }: Self::SystemData,
    ) {
        (
            &hp_bars,
            &parent_objects,
            &mut transforms,
            &mut sprite_renders,
        )
            .join()
            .filter_map(|(_, parent_object, transform, sprite_render)| {
                health_pointses
                    .get(parent_object.entity)
                    .map(|health_points| (transform, sprite_render, health_points))
            })
            .for_each(|(transform, sprite_render, health_points)| {
                let hp = (**health_points) as f32;

                // This is here because the `DrawFlat2D` pass renders sprites centered -- i.e. the
                // sprite is shifted left by half its width, and down by half its height.
                //
                // Since the `HpBar` is drawn centered, and we want it to be on the left in a fixed
                // position, we calculate how far it should be.
                let half_hp_lost = (HP_BAR_LENGTH - hp) / 2.;
                let translation = transform.translation_mut();
                translation.x += Float::from(-half_hp_lost);
                translation.y += Float::from(Y_OFFSET);
                translation.z += Float::from(Z_OFFSET);

                let scale = transform.scale_mut();
                scale[0] = Float::from(hp);

                sprite_render.sprite_number = (HP_BAR_SPRITE_COUNT - 1)
                    * ((**health_points) as usize)
                    / HP_BAR_LENGTH as usize;
            });
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::PrefabData,
        core::{math::Vector3, Float, Transform, TransformBundle},
        ecs::{Builder, Entity, System, SystemData},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use object_model::play::HealthPoints;

    use super::HpBarUpdateSystem;
    use crate::HpBarPrefab;

    #[test]
    fn sets_transform_x_and_scale() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                <HpBarPrefab as PrefabData>::SystemData::setup(&mut world.res);
                <HpBarUpdateSystem as System>::SystemData::setup(&mut world.res);

                let mut transform = Transform::default();
                transform.set_translation_x(123.);
                transform.set_translation_y(456.);
                transform.set_translation_z(789.);
                let char_entity = {
                    world
                        .create_entity()
                        .with(transform)
                        .with(HealthPoints::new(20))
                        .build()
                };

                let hp_bar_entity = {
                    let hp_bar_entity = world.create_entity().build();

                    let mut hp_bar_prefab_system_data =
                        world.system_data::<<HpBarPrefab as PrefabData>::SystemData>();
                    let hp_bar_prefab = HpBarPrefab::new(char_entity);

                    hp_bar_prefab
                        .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[], &[])
                        .expect("`HpBarPrefab` failed to augment entity.");

                    hp_bar_entity
                };

                world.add_resource(hp_bar_entity);
            })
            .with_system_single(HpBarUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                let hp_bar_entity = *world.read_resource::<Entity>();

                let transforms = world.read_storage::<Transform>();
                let transform = transforms
                    .get(hp_bar_entity)
                    .expect("Expected hp bar to have `Transform` component.");

                // 100 - 20 = 80 (80 HP)
                // -80 / 2  = -40 (half sprite width shift)
                // -40 + 123. = 83. (parent shift)
                assert_eq!(
                    &Vector3::new(Float::from(83.), Float::from(446.), Float::from(790.)),
                    transform.translation()
                );
                assert_eq!(Float::from(20.), transform.scale()[0]);
            })
            .run_isolated()
    }
}

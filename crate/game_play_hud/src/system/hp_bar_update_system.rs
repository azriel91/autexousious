use amethyst::{
    core::{Float, Parent, Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use derive_new::new;
use object_model::play::HealthPoints;
use typename_derive::TypeName;

use crate::{HpBar, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT};

/// Updates `HpBar` length based on its parent entity's `HealthPoints`.
#[derive(Debug, Default, TypeName, new)]
pub struct HpBarUpdateSystem;

type HpBarUpdateSystemData<'s> = (
    ReadStorage<'s, HpBar>,
    ReadStorage<'s, Parent>,
    ReadStorage<'s, HealthPoints>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, SpriteRender>,
);

impl<'s> System<'s> for HpBarUpdateSystem {
    type SystemData = HpBarUpdateSystemData<'s>;

    fn run(
        &mut self,
        (hp_bars, parents, health_pointses, mut transforms, mut sprite_renders): Self::SystemData,
    ) {
        (&hp_bars, &parents, &mut transforms, &mut sprite_renders)
            .join()
            .filter_map(|(_, parent, transform, sprite_render)| {
                health_pointses
                    .get(parent.entity)
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
                transform.set_translation_x(-half_hp_lost);

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
        core::{Float, Transform, TransformBundle},
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
                <HpBarUpdateSystem as System>::SystemData::setup(&mut world.res);

                let char_entity = {
                    world
                        .create_entity()
                        .with(Transform::default())
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

                // 100 - 20 = 80
                // -80 / 2  = -40
                assert_eq!(Float::from(-40.), transform.translation()[0]);
                assert_eq!(Float::from(20.), transform.scale()[0]);
            })
            .run_isolated()
    }
}

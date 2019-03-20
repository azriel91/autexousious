use amethyst::{
    core::{Parent, Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use derive_new::new;
use object_model::entity::HealthPoints;
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
                scale[0] = hp;

                sprite_render.sprite_number = (HP_BAR_SPRITE_COUNT - 1)
                    * ((**health_points) as usize)
                    / HP_BAR_LENGTH as usize;
            });
    }
}

// See tests/hp_bar_update_system.rs

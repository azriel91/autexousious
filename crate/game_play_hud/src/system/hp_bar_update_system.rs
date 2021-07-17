use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    renderer::SpriteRender,
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use object_model::play::HealthPoints;
use parent_model::play::ParentEntity;

use crate::{HpBar, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT};

/// Move HpBar below character.
const Y_OFFSET: f32 = -10.;
/// Move HpBar in front of object.
const Z_OFFSET: f32 = 1.;

/// Updates `HpBar` length based on its parent entity's `HealthPoints`.
#[derive(Debug, Default, new)]
pub struct HpBarUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct HpBarUpdateSystemData<'s> {
    /// `HpBar` components.
    #[derivative(Debug = "ignore")]
    pub hp_bars: ReadStorage<'s, HpBar>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: ReadStorage<'s, ParentEntity>,
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
            parent_entities,
            health_pointses,
            mut transforms,
            mut sprite_renders,
        }: Self::SystemData,
    ) {
        (
            &hp_bars,
            &parent_entities,
            &mut transforms,
            &mut sprite_renders,
        )
            .join()
            .filter_map(|(_, parent_entity, transform, sprite_render)| {
                health_pointses
                    .get(parent_entity.0)
                    .map(|health_points| (transform, sprite_render, health_points))
            })
            .for_each(|(transform, sprite_render, health_points)| {
                let hp = (**health_points) as f32;

                // This is here because the `DrawFlat2D` pass renders sprites centered -- i.e.
                // the sprite is shifted left by half its width, and down by
                // half its height.
                //
                // Since the `HpBar` is drawn centered, and we want it to be on the left in a
                // fixed position, we calculate how far it should be.
                let half_hp_lost = (HP_BAR_LENGTH - hp) / 2.;
                let translation = transform.translation_mut();
                translation.x += -half_hp_lost;
                translation.y += Y_OFFSET;
                translation.z += Z_OFFSET;

                let scale = transform.scale_mut();
                scale[0] = hp;

                sprite_render.sprite_number = (HP_BAR_SPRITE_COUNT - 1)
                    * ((**health_points) as usize)
                    / HP_BAR_LENGTH as usize;
            });
    }
}

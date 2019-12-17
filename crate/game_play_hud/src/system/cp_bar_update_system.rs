use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    renderer::SpriteRender,
    shred::{ResourceId, SystemData},
};
use charge_model::play::ChargeTrackerClock;
use derivative::Derivative;
use derive_new::new;
use parent_model::play::ParentObject;
use typename_derive::TypeName;

use crate::{CpBar, CP_BAR_LENGTH, CP_BAR_SPRITE_COUNT};

/// Move CpBar below character.
const Y_OFFSET: f32 = -14.;
/// Move CpBar in front of object.
const Z_OFFSET: f32 = 1.;

/// Updates `CpBar` length based on its parent entity's `ChargeTrackerClock`.
#[derive(Debug, Default, TypeName, new)]
pub struct CpBarUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CpBarUpdateSystemData<'s> {
    /// `CpBar` components.
    #[derivative(Debug = "ignore")]
    pub cp_bars: ReadStorage<'s, CpBar>,
    /// `ParentObject` components.
    #[derivative(Debug = "ignore")]
    pub parent_objects: ReadStorage<'s, ParentObject>,
    /// `ChargeTrackerClock` components.
    #[derivative(Debug = "ignore")]
    pub charge_tracker_clocks: ReadStorage<'s, ChargeTrackerClock>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
}

impl<'s> System<'s> for CpBarUpdateSystem {
    type SystemData = CpBarUpdateSystemData<'s>;

    fn run(
        &mut self,
        CpBarUpdateSystemData {
            cp_bars,
            parent_objects,
            charge_tracker_clocks,
            mut transforms,
            mut sprite_renders,
        }: Self::SystemData,
    ) {
        (
            &cp_bars,
            &parent_objects,
            &mut transforms,
            &mut sprite_renders,
        )
            .join()
            .filter_map(|(_, parent_object, transform, sprite_render)| {
                charge_tracker_clocks
                    .get(parent_object.entity)
                    .map(|charge_tracker_clock| (transform, sprite_render, charge_tracker_clock))
            })
            .for_each(|(transform, sprite_render, charge_tracker_clock)| {
                let cp_percentage = (*charge_tracker_clock).value as f32
                    / (*charge_tracker_clock).limit as f32
                    * CP_BAR_LENGTH;

                // This is here because the `DrawFlat2D` pass renders sprites centered -- i.e. the
                // sprite is shifted left by half its width, and down by half its height.
                //
                // Since the `CpBar` is drawn centered, and we want it to be on the left in a fixed
                // position, we calculate how far it should be.
                let half_cp_missing = (CP_BAR_LENGTH - cp_percentage) / 2.;
                let translation = transform.translation_mut();
                translation.x += -half_cp_missing;
                translation.y += Y_OFFSET;
                translation.z += Z_OFFSET;

                let scale = transform.scale_mut();
                scale[0] = cp_percentage;

                sprite_render.sprite_number = (CP_BAR_SPRITE_COUNT - 1)
                    * (*charge_tracker_clock).value
                    / CP_BAR_LENGTH as usize;
            });
    }
}

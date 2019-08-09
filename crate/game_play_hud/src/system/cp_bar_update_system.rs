use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use charge_model::play::ChargeTrackerClock;
use derivative::Derivative;
use derive_new::new;
use object_model::play::ParentObject;
use shred_derive::SystemData;
use typename_derive::TypeName;

use crate::{CpBar, CP_BAR_LENGTH, CP_BAR_SPRITE_COUNT};

/// Move CpBar below character.
const Y_OFFSET: f32 = -13.;
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

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::PrefabData,
        core::{math::Vector3, Transform, TransformBundle},
        ecs::{Builder, Entity, System, SystemData},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::ChargeTrackerClock;

    use super::CpBarUpdateSystem;
    use crate::CpBarPrefab;

    #[test]
    fn sets_transform_x_and_scale() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                <CpBarPrefab as PrefabData>::SystemData::setup(&mut world.res);
                <CpBarUpdateSystem as System>::SystemData::setup(&mut world.res);

                let mut transform = Transform::default();
                transform.set_translation_x(123.);
                transform.set_translation_y(456.);
                transform.set_translation_z(789.);

                let charge_tracker_clock = ChargeTrackerClock::new_with_value(20, 18);
                let char_entity = {
                    world
                        .create_entity()
                        .with(transform)
                        .with(charge_tracker_clock)
                        .build()
                };

                let cp_bar_entity = {
                    let cp_bar_entity = world.create_entity().build();

                    let mut cp_bar_prefab_system_data =
                        world.system_data::<<CpBarPrefab as PrefabData>::SystemData>();
                    let cp_bar_prefab = CpBarPrefab::new(char_entity);

                    cp_bar_prefab
                        .add_to_entity(cp_bar_entity, &mut cp_bar_prefab_system_data, &[], &[])
                        .expect("`CpBarPrefab` failed to augment entity.");

                    cp_bar_entity
                };

                world.add_resource(cp_bar_entity);
            })
            .with_system_single(CpBarUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                let cp_bar_entity = *world.read_resource::<Entity>();

                let transforms = world.read_storage::<Transform>();
                let transform = transforms
                    .get(cp_bar_entity)
                    .expect("Expected cp bar to have `Transform` component.");

                //  20 -   18  =   2
                //   2. /  20. =  10. (10%)
                // -10  /   2. =  -5. (half sprite width shift)
                //  -5. + 123. = 118. (parent shift)
                assert_eq!(&Vector3::new(118., 443., 790.), transform.translation());
                assert_eq!(90., transform.scale()[0]);
            })
            .run_isolated()
    }
}

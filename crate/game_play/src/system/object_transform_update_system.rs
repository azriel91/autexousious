use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::{config::Position, play::PositionZAsY};
use object_model::play::Mirrored;
use typename_derive::TypeName;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectTransformUpdateSystem;

/// `ObjectTransformUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectTransformUpdateSystemData<'s> {
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: ReadStorage<'s, SpriteRender>,
    /// `PositionZAsY` components.
    #[derivative(Debug = "ignore")]
    pub position_z_as_ys: ReadStorage<'s, PositionZAsY>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(
        &mut self,
        ObjectTransformUpdateSystemData {
            positions,
            mirroreds,
            sprite_renders,
            position_z_as_ys,
            sprite_sheet_assets,
            mut transforms,
        }: Self::SystemData,
    ) {
        for (position, mirrored, sprite_render, position_z_as_y, transform) in (
            &positions,
            mirroreds.maybe(),
            sprite_renders.maybe(),
            position_z_as_ys.maybe(),
            &mut transforms,
        )
            .join()
        {
            // Hack: Visual correction when sprites are mirrored.
            if let (Some(mirrored), Some(sprite_render)) = (mirrored, sprite_render) {
                if mirrored.0 {
                    let sprite_sheet = sprite_sheet_assets
                        .get(&sprite_render.sprite_sheet)
                        .expect("Expected sprite sheet to be loaded.");
                    let sprite = &sprite_sheet.sprites[sprite_render.sprite_number];
                    transform.set_translation_x(position.x + sprite.offsets[0]);
                } else {
                    transform.set_translation_x(position.x);
                }
            } else {
                transform.set_translation_x(position.x);
            }

            if position_z_as_y.is_some() {
                // We subtract z from the y translation as the z axis increases "out of the screen".
                // Entities that have a larger Z value are transformed downwards.
                transform.set_translation_y(position.y - position.z);
            } else {
                transform.set_translation_y(position.y);
            }
            transform.set_translation_z(position.z);
        }
    }
}

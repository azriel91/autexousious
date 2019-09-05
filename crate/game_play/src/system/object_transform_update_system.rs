use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    renderer::{camera::Camera, SpriteRender, SpriteSheet},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use object_model::play::Mirrored;
use typename_derive::TypeName;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectTransformUpdateSystem;

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
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: ReadStorage<'s, Camera>,
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
            cameras,
            sprite_sheet_assets,
            mut transforms,
        }: Self::SystemData,
    ) {
        for (position, mirrored, sprite_render, camera, transform) in (
            &positions,
            mirroreds.maybe(),
            sprite_renders.maybe(),
            cameras.maybe(),
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

            if camera.is_some() {
                transform.set_translation_y(position.y);
            } else {
                // We subtract z from the y translation as the z axis increases "out of the screen".
                // Entities that have a larger Z value are transformed downwards.
                transform.set_translation_y(position.y - position.z);
            }
            transform.set_translation_z(position.z);
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, WorldExt},
        input::StringBindings,
        renderer::camera::Camera,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Position;
    use typename::TypeName;

    use super::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                camera: None,
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -11., 1.)),
            },
        )
    }

    #[test]
    fn updates_camera_transform_xyz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::new(100., -10., 1.),
                camera: Some(Camera::standard_2d(800., 600.)),
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(100., -10., 1.)),
            },
        )
    }

    fn run_test(
        SetupParams { position, camera }: SetupParams,
        ExpectedParams {
            transform: transform_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_system(
                ObjectTransformUpdateSystem::new(),
                ObjectTransformUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(position)
                        .with(Transform::default());

                    if let Some(camera) = camera.clone() {
                        entity_builder = entity_builder.with(camera);
                    }

                    entity_builder.build()
                };

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let transforms = world.read_storage::<Transform>();

                let transform_actual = transforms
                    .get(entity)
                    .expect("Expected entity to have `Transform` component.");
                assert_eq!(
                    transform_expected.translation(),
                    transform_actual.translation()
                );
            })
            .run()
    }

    struct SetupParams {
        position: Position<f32>,
        camera: Option<Camera>,
    }
    struct ExpectedParams {
        transform: Transform,
    }
}

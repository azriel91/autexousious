use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WorldExt, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};
use derive_new::new;
use kinematic_model::config::Position;
use object_model::play::Mirrored;
use typename_derive::TypeName;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectTransformUpdateSystem;

type ObjectTransformUpdateSystemData<'s> = (
    ReadStorage<'s, Position<f32>>,
    ReadStorage<'s, Mirrored>,
    ReadStorage<'s, SpriteRender>,
    Read<'s, AssetStorage<SpriteSheet>>,
    WriteStorage<'s, Transform>,
);

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(
        &mut self,
        (positions, mirroreds, sprite_renders, sprite_sheet_assets, mut transform_storage): Self::SystemData,
    ) {
        for (position, mirrored, sprite_render, transform) in (
            &positions,
            mirroreds.maybe(),
            sprite_renders.maybe(),
            &mut transform_storage,
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

            // We subtract z from the y translation as the z axis increases "out of the screen".
            // Entities that have a larger Z value are transformed downwards.
            transform.set_translation_y(position.y - position.z);
            transform.set_translation_z(position.z);
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, World, WorldExt},
        input::StringBindings,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::Position;
    use typename::TypeName;

    use super::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz() -> Result<(), Error> {
        let setup = |world: &mut World| {
            // Create entity with position
            let position = Position::<f32>::new(-5., -3., -4.);

            let mut transform = Transform::default();
            transform.set_translation(Vector3::new(10., 20., 0.));

            let entity = world.create_entity().with(position).with(transform).build();

            world.insert(entity);
        };

        let assertion = |world: &mut World| {
            let entity = *world.read_resource::<Entity>();
            let transforms = world.read_storage::<Transform>();

            let expected_translation = Vector3::new(-5., 1., -4.);

            let transform = transforms
                .get(entity)
                .expect("Expected entity to have `Transform` component.");
            assert_eq!(&expected_translation, transform.translation());
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_system(
                ObjectTransformUpdateSystem::new(),
                ObjectTransformUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(setup)
            .with_assertion(assertion)
            .run()
    }
}

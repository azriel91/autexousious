use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};
use derivative::Derivative;
use derive_new::new;
use object_model::play::ParentObject;
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates a child object's translation to match its `ParentObject`'s translation.
///
/// If we use the `Parent` component, the child object will inherit all transformations, whereas
/// this will only copy over the **XYZ** coordinates.
#[derive(Debug, Default, TypeName, new)]
pub struct StickToParentObjectSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StickToParentObjectSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ParentObject` components.
    #[derivative(Debug = "ignore")]
    pub parent_objects: ReadStorage<'s, ParentObject>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for StickToParentObjectSystem {
    type SystemData = StickToParentObjectSystemData<'s>;

    fn run(
        &mut self,
        StickToParentObjectSystemData {
            entities,
            parent_objects,
            mut transforms,
        }: Self::SystemData,
    ) {
        (&entities, &parent_objects)
            .join()
            .for_each(|(child_entity, parent_object)| {
                let parent_translation = transforms
                    .get(parent_object.entity)
                    .map(Transform::translation)
                    .copied();

                if let Some(translation) = parent_translation {
                    if let Some(transform) = transforms.get_mut(child_entity) {
                        *transform.translation_mut() = translation;
                    }
                }
            });
    } // kcov-ignore
}

#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::Vector3, transform::Transform, Float},
        ecs::{Builder, Entity, World},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use object_model::play::ParentObject;

    use super::StickToParentObjectSystem;

    #[test]
    fn updates_child_translation_to_match_parent() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToParentObjectSystem::new(), "", &[])
            .with_setup(|world| create_parent_and_child_entity(world))
            .with_effect(|world| set_parent_translation(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_translation(world, 1., 2., 3.5))
            .run()
    }

    fn create_parent_and_child_entity(world: &mut World) {
        let parent = world.create_entity().with(Transform::default()).build();
        let child = world
            .create_entity()
            .with(Transform::default())
            .with(ParentObject::new(parent))
            .build();
        world.add_resource((parent, child));
    }

    fn set_parent_translation(
        world: &mut World,
        x: impl Into<Float>,
        y: impl Into<Float>,
        z: impl Into<Float>,
    ) {
        let (parent, _child) = *world.read_resource::<(Entity, Entity)>();
        let mut transforms = world.write_storage::<Transform>();
        let parent_transform = transforms
            .get_mut(parent)
            .expect("Expected parent entity to have `Transform` component.");

        parent_transform.set_translation_xyz(x, y, z);
    }

    fn assert_child_entity_translation(
        world: &mut World,
        x: impl Into<Float>,
        y: impl Into<Float>,
        z: impl Into<Float>,
    ) {
        let (_parent, child) = *world.read_resource::<(Entity, Entity)>();
        let transforms = world.read_storage::<Transform>();
        let child_transform = transforms
            .get(child)
            .expect("Expected child entity to have `Transform` component.");

        assert_eq!(
            &Vector3::new(x.into(), y.into(), z.into()),
            child_transform.translation()
        );
    }
}

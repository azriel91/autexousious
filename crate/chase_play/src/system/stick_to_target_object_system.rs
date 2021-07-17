use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use chase_model::play::{ChaseModeStick, TargetObject};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;

/// Updates a `ChaseModeStick` entity's `Position` and `Translation` to match
/// its `TargetObject`.
///
/// If we use the `Parent` component, the child object will inherit all
/// transformations, whereas this will only copy over the **XYZ** coordinates.
#[derive(Debug, Default, new)]
pub struct StickToTargetObjectSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StickToTargetObjectSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: ReadStorage<'s, TargetObject>,
    /// `ChaseModeStick` components.
    #[derivative(Debug = "ignore")]
    pub chase_mode_sticks: ReadStorage<'s, ChaseModeStick>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for StickToTargetObjectSystem {
    type SystemData = StickToTargetObjectSystemData<'s>;

    fn run(
        &mut self,
        StickToTargetObjectSystemData {
            entities,
            target_objects,
            chase_mode_sticks,
            mut positions,
            mut transforms,
        }: Self::SystemData,
    ) {
        (&entities, &target_objects, &chase_mode_sticks)
            .join()
            .for_each(|(child_entity, target_object, chase_mode_stick)| {
                let target_position = positions.get(target_object.entity).copied();
                let offset = chase_mode_stick.offset;

                if let Some(target_position) = target_position {
                    if let Some(position) = positions.get_mut(child_entity) {
                        *position = target_position;

                        if let Some(offset) = offset {
                            *position += offset;
                        }
                    }
                }

                let target_translation = transforms
                    .get(target_object.entity)
                    .map(Transform::translation)
                    .copied();

                if let Some(translation) = target_translation {
                    if let Some(transform) = transforms.get_mut(child_entity) {
                        let translation = if let Some(offset) = offset {
                            translation + offset.0
                        } else {
                            translation
                        };
                        *transform.translation_mut() = translation;
                    }
                }
            });
    } // kcov-ignore
}

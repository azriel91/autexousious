use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use log::warn;
use serde::{Deserialize, Serialize};

/// Scale initializer for an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Component, PartialEq, Serialize, new)]
#[serde(default)]
#[storage(DenseVecStorage)]
pub struct ScaleInit {
    /// Initial X scaling.
    pub x: f32,
    /// Initial Y scaling.
    pub y: f32,
    /// Initial Z scaling.
    pub z: f32,
}

/// `ScaleInitSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ScaleInitSystemData<'s> {
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> ItemComponent<'s> for ScaleInit {
    type SystemData = ScaleInitSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let ScaleInitSystemData { transforms } = system_data;

        match transforms.get_mut(entity) {
            Some(transform) => {
                transform.set_scale(Vector3::new(self.x, self.y, self.z));
            }
            None => warn!(
                "`ScaleInit` `{:?}` attached to entity without `Transform` component: {:?}",
                self, entity
            ),
        }
    }
}

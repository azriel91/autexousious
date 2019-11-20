use std::ops::{Add, AddAssign};

use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};
use typename_derive::TypeName;

use crate::config::Position;

/// Position initializer for an entity.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Component, PartialEq, Eq, Serialize, TypeName, new,
)]
#[serde(default)]
#[storage(DenseVecStorage)]
pub struct PositionInit {
    /// Initial X coordinate.
    pub x: i32,
    /// Initial Y coordinate.
    pub y: i32,
    /// Initial Z coordinate.
    pub z: i32,
}

impl Add for PositionInit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for PositionInit {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Into<Vector3<f32>> for PositionInit {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// `PositionInitSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct PositionInitSystemData<'s> {
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> ItemComponent<'s> for PositionInit {
    type SystemData = PositionInitSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let PositionInitSystemData {
            positions,
            transforms,
        } = system_data;

        let translation = Into::<Vector3<f32>>::into(*self);
        let position = Position::from(translation);
        let mut transform = Transform::default();
        transform.set_translation(translation);

        positions
            .insert(entity, position)
            .expect("Failed to insert `Position<f32>` component.");
        transforms
            .insert(entity, transform)
            .expect("Failed to insert `Transform` component.");
    }
}

use std::ops::{Add, AddAssign, Mul, MulAssign};

use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{storage::DenseVecStorage, Component, Entity, ReadStorage, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Position, ScaleInit},
    play::{PositionInitOffset, PositionInitParent},
};

/// Position initializer for an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Component, PartialEq, Eq, Serialize, new)]
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

impl Mul<f32> for PositionInit {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: (self.x as f32 * other) as i32,
            y: (self.y as f32 * other) as i32,
            z: (self.z as f32 * other) as i32,
        }
    }
}

impl MulAssign<f32> for PositionInit {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: (self.x as f32 * other) as i32,
            y: (self.y as f32 * other) as i32,
            z: (self.z as f32 * other) as i32,
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
    /// `PositionInitParent` components.
    #[derivative(Debug = "ignore")]
    pub position_init_parents: ReadStorage<'s, PositionInitParent>,
    /// `PositionInitOffset` components.
    #[derivative(Debug = "ignore")]
    pub position_init_offsets: ReadStorage<'s, PositionInitOffset>,
    /// `ScaleInit` components.
    #[derivative(Debug = "ignore")]
    pub scale_inits: ReadStorage<'s, ScaleInit>,
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
            position_init_parents,
            position_init_offsets,
            scale_inits,
            positions,
            transforms,
        } = system_data;

        // Get parent position if any.
        let position_parent = position_init_parents
            .get(entity)
            .and_then(|position_init_parent| positions.get(position_init_parent.0).copied());
        let position_init_offset = position_init_offsets.get(entity);
        let scale_init = scale_inits.get(entity).copied();

        let mut translation = Into::<Position<f32>>::into(*self);
        if let Some(position_init_offset) = position_init_offset {
            let mut position_init_offset = **position_init_offset;

            // Since scaling affects the whole transform, we need to divide the offset by the scale
            // factor to retain the original values.
            if let Some(scale_init) = scale_init {
                position_init_offset.x /= scale_init.x;
                position_init_offset.y /= scale_init.y;
                position_init_offset.z /= scale_init.z;
            }

            translation += position_init_offset;
        }
        let mut translation = Into::<Vector3<f32>>::into(translation);
        if let Some(scale_init) = scale_init {
            translation.x *= scale_init.x;
            translation.y *= scale_init.y;
            translation.z *= scale_init.z;
        }

        if let Some(position_parent) = position_parent {
            translation += *position_parent;
        }

        let position = Position::from(translation);
        let mut transform = Transform::default();
        transform.set_translation(translation);

        if let Some(scale_init) = scale_init {
            transform.set_scale(Vector3::new(scale_init.x, scale_init.y, scale_init.z));
        }

        if positions.get(entity).is_none() {
            positions
                .insert(entity, position)
                .expect("Failed to insert `Position<f32>` component.");
        }
        if transforms.get(entity).is_none() {
            transforms
                .insert(entity, transform)
                .expect("Failed to insert `Transform` component.");
        }
    }
}

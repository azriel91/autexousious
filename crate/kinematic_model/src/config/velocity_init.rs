use std::ops::{Add, AddAssign};

use amethyst::{
    core::math::Vector3,
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Velocity;

/// Velocity initializer for an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Component, PartialEq, Eq, Serialize, new)]
#[serde(default)]
#[storage(DenseVecStorage)]
pub struct VelocityInit {
    /// Initial X velocity.
    pub x: i32,
    /// Initial Y velocity.
    pub y: i32,
    /// Initial Z velocity.
    pub z: i32,
}

impl Add for VelocityInit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for VelocityInit {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Into<Vector3<f32>> for VelocityInit {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// `VelocityInitSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct VelocityInitSystemData<'s> {
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl<'s> ItemComponent<'s> for VelocityInit {
    type SystemData = VelocityInitSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let VelocityInitSystemData { velocities } = system_data;

        let velocity = Velocity::from(Into::<Vector3<f32>>::into(*self));

        if velocities.get(entity).is_none() {
            velocities
                .insert(entity, velocity)
                .expect("Failed to insert `Velocity<f32>` component.");
        }
    }
}

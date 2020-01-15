//! Contains the types that represent the configuration on disk.

pub use self::{
    object_acceleration::ObjectAcceleration, object_acceleration_kind::ObjectAccelerationKind,
    object_acceleration_value::ObjectAccelerationValue,
    object_acceleration_value_expr::ObjectAccelerationValueExpr,
    object_acceleration_value_multiplier::ObjectAccelerationValueMultiplier,
    position_init::PositionInit, vector3::Vector3, velocity_init::VelocityInit,
};

mod object_acceleration;
mod object_acceleration_kind;
mod object_acceleration_value;
mod object_acceleration_value_expr;
mod object_acceleration_value_multiplier;
mod position_init;
mod vector3;
mod velocity_init;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

use amethyst::{
    core::math,
    ecs::{storage::DenseVecStorage, Component},
};
use serde::{Deserialize, Serialize};

macro_rules! kinematic_type {
    ($name:ident) => {
        /// #[doc = $name]
        /// of the entity in game.
        #[derive(Clone, Component, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
        #[serde(from = "Vector3<S>", into = "Vector3<S>")]
        pub struct $name<S>(pub math::Vector3<S>)
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static;

        impl<S> $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            /// Returns a new `
            /// #[doc = $name]
            /// `.
            pub fn new(x: S, y: S, z: S) -> Self {
                $name(math::Vector3::new(x, y, z))
            }
        }

        impl<S> Default for $name<S>
        where
            S: Clone + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn default() -> Self {
                $name(math::Vector3::new(S::default(), S::default(), S::default()))
            }
        }

        impl<S> From<Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: Vector3<S>) -> Self {
                $name::new(v.x, v.y, v.z)
            }
        }

        impl<S> Into<Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn into(self) -> Vector3<S> {
                Vector3::new(self.0.x, self.0.y, self.0.z)
            }
        }

        impl<S> From<math::Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: math::Vector3<S>) -> Self {
                $name(v)
            }
        }

        impl<S> Into<math::Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn into(self) -> math::Vector3<S> {
                self.0
            }
        }

        impl<'a, S> From<&'a math::Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a math::Vector3<S>) -> Self {
                $name(*v)
            }
        }

        impl<'a, S> From<&'a mut math::Vector3<S>> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a mut math::Vector3<S>) -> Self {
                $name(*v)
            }
        }

        impl<S> From<(S, S, S)> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: (S, S, S)) -> Self {
                $name::new(v.0, v.1, v.2)
            }
        }

        impl<'a, S> From<&'a (S, S, S)> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a (S, S, S)) -> Self {
                $name::new(v.0, v.1, v.2)
            }
        }

        impl<'a, S> From<&'a mut (S, S, S)> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a mut (S, S, S)) -> Self {
                $name::new(v.0, v.1, v.2)
            }
        }

        impl<S> From<[S; 3]> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: [S; 3]) -> Self {
                $name::new(v[0], v[1], v[2])
            }
        }

        impl<'a, S> From<&'a [S; 3]> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a [S; 3]) -> Self {
                $name::new(v[0], v[1], v[2])
            }
        }

        impl<'a, S> From<&'a mut [S; 3]> for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn from(v: &'a mut [S; 3]) -> Self {
                $name::new(v[0], v[1], v[2])
            }
        }

        impl<S> Deref for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            type Target = math::Vector3<S>;

            fn deref(&self) -> &math::Vector3<S> {
                &self.0
            }
        }

        impl<S> DerefMut for $name<S>
        where
            S: Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn deref_mut(&mut self) -> &mut math::Vector3<S> {
                &mut self.0
            }
        }

        impl<S> Add for $name<S>
        where
            S: Add<Output = S> + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        impl<S> AddAssign for $name<S>
        where
            S: Add<Output = S> + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn add_assign(&mut self, other: Self) {
                *self = Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
            }
        }

        impl<S> Sub for $name<S>
        where
            S: Sub<Output = S> + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        impl<S> SubAssign for $name<S>
        where
            S: Sub<Output = S> + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
        {
            fn sub_assign(&mut self, other: Self) {
                *self = Self::new(self.x - other.x, self.y - other.y, self.z - other.z);
            }
        }
    };
}

kinematic_type!(Position);
kinematic_type!(Velocity);
kinematic_type!(Acceleration);

impl From<PositionInit> for Position<f32> {
    fn from(position_init: PositionInit) -> Self {
        // Note: `i32` as `f32` is a lossy conversion, which is why we cannot implement this
        // generically with `S: From<i32>`, as `f32` is not `From<i32>`.
        Position::new(
            position_init.x as f32,
            position_init.y as f32,
            position_init.z as f32,
        )
    }
}

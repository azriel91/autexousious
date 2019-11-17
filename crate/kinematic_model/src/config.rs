//! Contains the types that represent the configuration on disk.

pub use self::{
    object_acceleration::ObjectAcceleration, object_acceleration_kind::ObjectAccelerationKind,
    object_acceleration_value::ObjectAccelerationValue,
    object_acceleration_value_expr::ObjectAccelerationValueExpr,
    object_acceleration_value_multiplier::ObjectAccelerationValueMultiplier,
    position_init::PositionInit, vector3::Vector3,
};

mod object_acceleration;
mod object_acceleration_kind;
mod object_acceleration_value;
mod object_acceleration_value_expr;
mod object_acceleration_value_multiplier;
mod position_init;
mod vector3;

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
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
    };
}

kinematic_type!(Position);
kinematic_type!(Velocity);
kinematic_type!(Acceleration);

use amethyst::{
    core::math::Vector3,
    ecs::{storage::DenseVecStorage, Component},
};
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};

/// Target coordinates for the camera to move towards.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, Derivative, PartialEq)]
#[derivative(Default)]
pub struct CameraTargetCoordinates(
    #[derivative(Default(value = "Vector3::new(0., 0., 0.)"))] pub Vector3<f32>,
);

impl CameraTargetCoordinates {
    /// Returns a new `CameraTargetCoordinates`.
    pub fn new(x: f32, y: f32, z: f32) -> CameraTargetCoordinates {
        CameraTargetCoordinates(Vector3::new(x, y, z))
    }
}

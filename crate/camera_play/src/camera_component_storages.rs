use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
    utils::ortho_camera::CameraOrtho,
};
use camera_model::play::CameraTargetCoordinates;
use derivative::Derivative;
use kinematic_model::config::{Position, Velocity};

/// Camera entity `Component` storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CameraComponentStorages<'s> {
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: WriteStorage<'s, Camera>,
    /// `CameraOrtho` components.
    #[derivative(Debug = "ignore")]
    pub camera_orthos: WriteStorage<'s, CameraOrtho>,
    /// `CameraTargetCoordinates` components.
    #[derivative(Debug = "ignore")]
    pub camera_target_coordinateses: WriteStorage<'s, CameraTargetCoordinates>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

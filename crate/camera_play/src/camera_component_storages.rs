use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
    utils::ortho_camera::CameraOrtho,
};
use derivative::Derivative;

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
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

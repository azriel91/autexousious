use amethyst::{
    ecs::{Read, ReadExpect, World},
    shred::{ResourceId, SystemData},
    window::ScreenDimensions,
};
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;

use crate::CameraComponentStorages;

/// Camera entity `Component` storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CameraCreatorResources<'s> {
    /// `ScreenDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub screen_dimensions: ReadExpect<'s, ScreenDimensions>,
    /// `CameraZoomDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub camera_zoom_dimensions: Read<'s, CameraZoomDimensions>,
    /// `CameraComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub camera_component_storages: CameraComponentStorages<'s>,
}

use derivative::Derivative;
use derive_new::new;

/// Play area visible width.
pub const CAMERA_ZOOM_WIDTH_DEFAULT: f32 = 800.;
/// Play area visible height.
pub const CAMERA_ZOOM_HEIGHT_DEFAULT: f32 = 600.;
/// Play area visible depth.
pub const CAMERA_ZOOM_DEPTH_DEFAULT: f32 = 2000.;

/// Dimensions of the playable area that is in view.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, new)]
#[derivative(Default)]
pub struct CameraZoomDimensions {
    /// Width of the playable area that is in view.
    #[derivative(Default(value = "CAMERA_ZOOM_WIDTH_DEFAULT"))]
    pub width: f32,
    /// Height of the playable area that is in view.
    #[derivative(Default(value = "CAMERA_ZOOM_HEIGHT_DEFAULT"))]
    pub height: f32,
    /// Depth of the playable area that is in view.
    #[derivative(Default(value = "CAMERA_ZOOM_DEPTH_DEFAULT"))]
    pub depth: f32,
}

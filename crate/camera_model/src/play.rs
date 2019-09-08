//! Contains data types used during game play.

pub use self::{
    camera_target_coordinates::CameraTargetCoordinates,
    camera_tracked::CameraTracked,
    camera_zoom_dimensions::{
        CameraZoomDimensions, CAMERA_ZOOM_DEPTH_DEFAULT, CAMERA_ZOOM_HEIGHT_DEFAULT,
        CAMERA_ZOOM_WIDTH_DEFAULT,
    },
};

mod camera_target_coordinates;
mod camera_tracked;
mod camera_zoom_dimensions;

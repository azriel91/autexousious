use amethyst::{
    core::math::Vector3,
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
};
use camera_model::play::{CameraTargetCoordinates, CameraTracked, CameraZoomDimensions};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use map_model::{
    config::MapBounds,
    loaded::{AssetMapBounds, AssetMargins, Margins},
};
use map_selection_model::MapSelection;
use object_model::play::Mirrored;
use typename_derive::TypeName;

/// Focuses the camera at the average position of tracked entities.
#[derive(Debug, Default, TypeName, new)]
pub struct CameraTrackingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CameraTrackingSystemData<'s> {
    /// `CameraZoomDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub camera_zoom_dimensions: Read<'s, CameraZoomDimensions>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `CameraTracked` components.
    #[derivative(Debug = "ignore")]
    pub camera_trackeds: ReadStorage<'s, CameraTracked>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: ReadStorage<'s, Camera>,
    /// `CameraTargetCoordinates` components.
    #[derivative(Debug = "ignore")]
    pub camera_target_coordinateses: WriteStorage<'s, CameraTargetCoordinates>,
}

impl CameraTrackingSystem {
    /// Returns the mean position of `CameraTracked` entities.
    fn position_average(
        camera_trackeds: &ReadStorage<'_, CameraTracked>,
        positions: &ReadStorage<'_, Position<f32>>,
    ) -> Vector3<f32> {
        let positions = (camera_trackeds, positions)
            .join()
            .map(|(_, position)| **position)
            .collect::<Vec<Vector3<f32>>>();

        positions.iter().sum::<Vector3<f32>>() / (positions.len() as f32)
    }

    /// Returns the coordinates for the camera to focus on the average position.
    fn camera_target_coordinates(
        map_margins: Margins,
        map_bounds: MapBounds,
        camera_zoom_dimensions: CameraZoomDimensions,
        focus_coordinate: Vector3<f32>,
    ) -> CameraTargetCoordinates {
        let x_centred = if camera_zoom_dimensions.width < map_bounds.width as f32 {
            focus_coordinate
                .x
                .max(map_margins.left + camera_zoom_dimensions.width / 2.)
                .min(map_margins.right - camera_zoom_dimensions.width / 2.)
        } else {
            camera_zoom_dimensions.width / 2.
        };
        let y_centred =
            if camera_zoom_dimensions.height < (map_bounds.height + map_bounds.depth) as f32 {
                // Subtract Z because Z+ is rendered downwards.
                let yz_avg = focus_coordinate.y - focus_coordinate.z;
                let bounded_max = map_margins.top
                    - map_margins.back
                    - map_bounds.depth as f32
                    - camera_zoom_dimensions.height / 2.;
                let bounded_min = map_margins.bottom - map_margins.front - map_bounds.depth as f32
                    + camera_zoom_dimensions.height / 2.;

                yz_avg.max(bounded_min).min(bounded_max)
            } else {
                camera_zoom_dimensions.height / 2.
            };
        let z_centred = focus_coordinate.z + camera_zoom_dimensions.depth / 2.;

        CameraTargetCoordinates::new(x_centred, y_centred, z_centred)
    }

    /// Returns the position skewed in the direction tracked entities are facing.
    fn position_with_direction(
        camera_trackeds: &ReadStorage<'_, CameraTracked>,
        mirroreds: &ReadStorage<'_, Mirrored>,
        camera_zoom_dimensions: CameraZoomDimensions,
        mut target_position: Vector3<f32>,
    ) -> Vector3<f32> {
        let mirroreds = (camera_trackeds, mirroreds)
            .join()
            .map(|(_, mirrored)| *mirrored)
            .collect::<Vec<Mirrored>>();

        if !mirroreds.is_empty() {
            // We take half the length and subtract number of mirrored entities.
            //
            // This determines the factor to multiply the offset with, which adjusts how
            // much weighting we place on the offset to shift left or right based on the
            // direction that the `CameraTracked` entities are facing.
            let mirrored_count = mirroreds.iter().filter(|mirrored| mirrored.0).count() as f32;
            let mirrored_weight = mirroreds.len() as f32 / 2. - mirrored_count;

            // An offset of 1/6th of camera zoom width will keep the tracked average
            // position at 1/3rd of the window -- since it is already offset to the centre
            // of the window.

            // 1/6 * mirrored_weight * zoom width.
            let offset = mirrored_weight * camera_zoom_dimensions.width / 6.;

            target_position.x += offset
        }

        target_position
    }
}

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = CameraTrackingSystemData<'s>;

    fn run(
        &mut self,
        CameraTrackingSystemData {
            camera_zoom_dimensions,
            map_selection,
            asset_map_bounds,
            asset_margins,
            camera_trackeds,
            positions,
            mirroreds,
            cameras,
            mut camera_target_coordinateses,
        }: Self::SystemData,
    ) {
        let map_asset_id = map_selection
            .asset_id()
            .expect("Expected `MapSelection` asset ID to exist.");
        let map_margins = asset_margins
            .get(map_asset_id)
            .copied()
            .expect("Expected `Margins` to be loaded.");
        let map_bounds = asset_map_bounds
            .get(map_asset_id)
            .copied()
            .expect("Expected `MapBounds` to be loaded.");

        // Focus on tracked entities.
        //
        // Keep the average x in the middle of the screen, offset by the direction characters are
        // facing.
        // Keep the average (y + z) in the middle of the screen.
        let position_avg = Self::position_average(&camera_trackeds, &positions);
        let target_position = Self::position_with_direction(
            &camera_trackeds,
            &mirroreds,
            *camera_zoom_dimensions,
            position_avg,
        );

        let target_coordinates = Self::camera_target_coordinates(
            map_margins,
            map_bounds,
            *camera_zoom_dimensions,
            target_position,
        );

        (&cameras, &mut camera_target_coordinateses)
            .join()
            .for_each(|(_, camera_target_coordinates)| {
                *camera_target_coordinates = target_coordinates;
            });
    }
}

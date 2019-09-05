use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Entity, SystemData, World},
    renderer::camera::{Camera, Projection},
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
};
use camera_model::play::{CameraTargetCoordinates, CAMERA_ZOOM_DEPTH_DEFAULT};
use kinematic_model::config::Position;

use crate::{CameraComponentStorages, CameraCreatorResources};

/// Creates camera entities.
#[derive(Debug)]
pub struct CameraCreator;

impl CameraCreator {
    /// Creates a camera entity in the given `World`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to create the camera entity in.
    pub fn create_in_world(world: &mut World) -> Entity {
        world.setup::<CameraCreatorResources<'_>>();

        CameraCreatorResources::setup(world);
        Self::create(&mut world.system_data::<CameraCreatorResources<'_>>())
    }

    /// Creates a camera entity.
    ///
    /// # Parameters
    ///
    /// * `camera_creator_resources`: Resources used to create the camera.
    pub fn create(
        CameraCreatorResources {
            entities,
            screen_dimensions,
            camera_zoom_dimensions,
            camera_component_storages:
                CameraComponentStorages {
                    cameras,
                    camera_orthos,
                    camera_target_coordinateses,
                    positions,
                    transforms,
                },
        }: &mut CameraCreatorResources<'_>,
    ) -> Entity {
        let (window_width, window_height) = (screen_dimensions.width(), screen_dimensions.height());
        let (zoom_width, zoom_height) =
            (camera_zoom_dimensions.width, camera_zoom_dimensions.height);

        let camera = Camera::from(Projection::orthographic(
            -window_width / 2.,
            window_width / 2.,
            -window_height / 2.,
            window_height / 2.,
            0.,
            // The distance that the camera can see. Since the camera is moved to a large Z
            // position, we also need to give it the same Z depth vision to ensure it can see all
            // entities in front of it.
            CAMERA_ZOOM_DEPTH_DEFAULT,
        ));
        let camera_ortho = {
            let world_coordinates = CameraOrthoWorldCoordinates {
                left: -zoom_width / 2.,
                right: zoom_width / 2.,
                bottom: -zoom_height / 2.,
                top: zoom_height / 2.,
            };
            let mut camera_ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
            camera_ortho.world_coordinates = world_coordinates;

            camera_ortho
        };

        let camera_target_coordinates = CameraTargetCoordinates::default();

        // Camera translation from origin.
        //
        // The Z coordinate of the camera is how far along it should be before it faces the
        // entities. If an entity's Z coordinate is greater than the camera's Z coordinate, it will
        // be culled.
        //
        // We cannot use `::std::f32::MAX`, because float inconsistencies cause nothing to be
        // rendered. We rely on `CAMERA_ZOOM_DEPTH_DEFAULT` being large enough to fit everything in
        // view.
        let translation = Vector3::new(
            zoom_width / 2.,
            zoom_height / 2.,
            CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
        );
        let position = Position::from(translation);
        let transform = Transform::from(translation);

        let entity = entities.create();

        cameras
            .insert(entity, camera)
            .expect("Failed to insert `Camera` component.");
        camera_orthos
            .insert(entity, camera_ortho)
            .expect("Failed to insert `CameraOrtho` component.");
        camera_target_coordinateses
            .insert(entity, camera_target_coordinates)
            .expect("Failed to insert `CameraTargetCoordinates` component.");
        positions
            .insert(entity, position)
            .expect("Failed to insert `Position<f32>` component.");
        transforms
            .insert(entity, transform)
            .expect("Failed to insert `Transform` component.");

        entity
    }
}

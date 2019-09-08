use amethyst::{
    assets::AssetStorage,
    core::math::Vector3,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
};
use camera_model::play::{CameraTargetCoordinates, CameraTracked, CameraZoomDimensions};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use map_model::{
    config::MapBounds,
    loaded::{Map, Margins},
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
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub map_assets: Read<'s, AssetStorage<Map>>,
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
            map_assets,
            camera_trackeds,
            positions,
            mirroreds,
            cameras,
            mut camera_target_coordinateses,
        }: Self::SystemData,
    ) {
        let map = map_assets
            .get(map_selection.handle())
            .expect("Expected map to be loaded.");
        let map_margins = map.margins;
        let map_bounds = map.definition.header.bounds;

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, System, SystemData, World, WorldExt},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
    use camera_model::play::{
        CameraTargetCoordinates, CameraTracked, CAMERA_ZOOM_DEPTH_DEFAULT,
        CAMERA_ZOOM_HEIGHT_DEFAULT, CAMERA_ZOOM_WIDTH_DEFAULT,
    };
    use kinematic_model::config::Position;
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::{MapBounds, MapDefinition, MapHeader},
        loaded::{Map, Margins},
    };
    use map_selection_model::MapSelection;
    use object_model::play::Mirrored;
    use pretty_assertions::assert_eq;
    use typename::TypeName;

    use super::CameraTrackingSystem;
    use crate::CameraCreator;

    // Use large values so that it is easier to test middle cases (non edge cases).
    const MAP_WIDTH: f32 = 4000.;
    const MAP_HEIGHT: f32 = 3000.;
    const MAP_DEPTH: f32 = 1000.;

    #[test]
    fn sets_camera_x_to_tracked_average_x() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 1500., 0.), None),
                    (Position::new(1100., 1500., 0.), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    1000.,
                    1500.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn sets_camera_y_to_tracked_average_yz() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 900., 500.), None),
                    (Position::new(1100., 1100., 700.), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    1000.,
                    400.,
                    600. + CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn does_not_go_out_of_bounds_left() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(0., 900., 500.), None),
                    (Position::new(0., 1100., 700.), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    CAMERA_ZOOM_WIDTH_DEFAULT / 2.,
                    400.,
                    600. + CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn does_not_go_out_of_bounds_right() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(MAP_WIDTH, 900., 500.), None),
                    (Position::new(MAP_WIDTH, 1100., 700.), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    MAP_WIDTH - CAMERA_ZOOM_WIDTH_DEFAULT / 2.,
                    400.,
                    600. + CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn does_not_go_out_of_bounds_top() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., MAP_HEIGHT, 0.), None),
                    (Position::new(1100., MAP_HEIGHT, 0.), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    1000.,
                    MAP_HEIGHT - CAMERA_ZOOM_HEIGHT_DEFAULT / 2.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn does_not_go_out_of_bounds_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 0., MAP_DEPTH), None),
                    (Position::new(1100., 0., MAP_DEPTH), None),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    1000.,
                    -MAP_DEPTH + CAMERA_ZOOM_HEIGHT_DEFAULT / 2.,
                    MAP_DEPTH + CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn centres_camera_if_map_dimensions_are_smaller_than_zoom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(0., 0., 0.), None),
                    (Position::new(0., 0., 0.), None),
                ],
                map: small_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    CAMERA_ZOOM_WIDTH_DEFAULT / 2.,
                    CAMERA_ZOOM_HEIGHT_DEFAULT / 2.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn centres_camera_if_map_dimensions_are_smaller_than_zoom_with_mirrored() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(0., 0., 0.), Some(Mirrored::new(true))),
                    (Position::new(0., 0., 0.), Some(Mirrored::new(true))),
                ],
                map: small_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    CAMERA_ZOOM_WIDTH_DEFAULT / 2.,
                    CAMERA_ZOOM_HEIGHT_DEFAULT / 2.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn sets_camera_x_to_x_offset_when_entities_have_mirrored_false() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 1500., 0.), Some(Mirrored::new(false))),
                    (Position::new(1100., 1500., 0.), Some(Mirrored::new(false))),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    // Shift target 1/6 to the right.
                    // 800. / 6 = 133.333
                    1000. + CAMERA_ZOOM_WIDTH_DEFAULT / 6.,
                    1500.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn sets_camera_x_to_x_offset_when_entities_have_mirrored_true() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 1500., 0.), Some(Mirrored::new(true))),
                    (Position::new(1100., 1500., 0.), Some(Mirrored::new(true))),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    // Shift target 1/6 to the left.
                    // 800. / 6 = 133.333
                    1000. - CAMERA_ZOOM_WIDTH_DEFAULT / 6.,
                    1500.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    #[test]
    fn sets_camera_x_to_x_offset_when_entities_have_mirrored_balanced() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_mirroreds: vec![
                    (Position::new(900., 1500., 0.), Some(Mirrored::new(true))),
                    (Position::new(1100., 1500., 0.), Some(Mirrored::new(false))),
                ],
                map: big_map(),
            },
            ExpectedParams {
                camera_target_coordinates: CameraTargetCoordinates::new(
                    1000.,
                    1500.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                ),
            },
        )
    }

    fn run_test(
        SetupParams {
            position_mirroreds,
            map,
        }: SetupParams,
        ExpectedParams {
            camera_target_coordinates,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(MapLoadingBundle::new())
            .with_effect(setup_system_data)
            .with_effect(move |world| {
                let map_handle = {
                    let loader = world.read_resource::<Loader>();
                    let map_assets = world.read_resource::<AssetStorage<Map>>();

                    loader.load_from_data(map.clone(), (), &map_assets)
                };

                let slug =
                    AssetSlug::from_str("test/big_map").expect("Expected asset slug to be valid.");
                let snh = SlugAndHandle::new(slug, map_handle);
                let map_selection = MapSelection::Id(snh);

                world.insert(map_selection);
            })
            .with_effect(|world| {
                let camera_entity = CameraCreator::create_in_world(world);
                world.insert(camera_entity);
            })
            .with_effect(move |world| {
                position_mirroreds.iter().for_each(|(position, mirrored)| {
                    let mut entity_builder = world
                        .create_entity()
                        .with(CameraTracked)
                        .with(position.clone());

                    if let Some(mirrored) = mirrored {
                        entity_builder = entity_builder.with(*mirrored);
                    }

                    entity_builder.build();
                });
            })
            .with_system_single(
                CameraTrackingSystem::new(),
                CameraTrackingSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let camera_target_coordinateses = world.read_storage::<CameraTargetCoordinates>();
                let camera_target_coordinates_actual = camera_target_coordinateses
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `CameraTargetCoordinates` component.");

                assert_eq!(camera_target_coordinates, camera_target_coordinates_actual);
            })
            .run()
    }

    fn small_map() -> Map {
        let map_bounds = MapBounds::new(0, 0, 0, 400, 300, 200);
        let map_header = MapHeader::new(String::from("small_map"), map_bounds);
        let map_definition = MapDefinition::new(map_header, Vec::new());
        let map_margins = Margins::from(map_bounds);
        Map::new(
            map_definition,
            map_margins,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    fn big_map() -> Map {
        let map_bounds = MapBounds::new(
            0,
            0,
            0,
            MAP_WIDTH as u32,
            MAP_HEIGHT as u32,
            MAP_DEPTH as u32,
        );
        let map_header = MapHeader::new(String::from("big_map"), map_bounds);
        let map_definition = MapDefinition::new(map_header, Vec::new());
        let map_margins = Margins::from(map_bounds);
        Map::new(
            map_definition,
            map_margins,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    fn setup_system_data(world: &mut World) {
        <CameraTrackingSystem as System<'_>>::SystemData::setup(world);
    }

    struct SetupParams {
        position_mirroreds: Vec<(Position<f32>, Option<Mirrored>)>,
        map: Map,
    }

    struct ExpectedParams {
        camera_target_coordinates: CameraTargetCoordinates,
    }
}

#[cfg(test)]
mod tests {
    use std::{any, str::FromStr};

    use amethyst::{
        ecs::{Builder, Entity, Read, System, SystemData, World, WorldExt},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use camera_model::play::{
        CameraTargetCoordinates, CameraTracked, CAMERA_ZOOM_DEPTH_DEFAULT,
        CAMERA_ZOOM_HEIGHT_DEFAULT, CAMERA_ZOOM_WIDTH_DEFAULT,
    };
    use kinematic_model::config::Position;
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::MapBounds,
        loaded::{AssetMapBounds, AssetMargins, Margins},
    };
    use map_selection_model::MapSelection;
    use mirrored_model::play::Mirrored;
    use pretty_assertions::assert_eq;

    use camera_play::{CameraCreator, CameraTrackingSystem};

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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_small_map,
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
                setup_map_selection_fn: setup_small_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
                setup_map_selection_fn: setup_big_map,
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
            setup_map_selection_fn,
        }: SetupParams,
        ExpectedParams {
            camera_target_coordinates,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(MapLoadingBundle::new())
            .with_effect(setup_system_data)
            .with_effect(setup_map_selection_fn)
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
                any::type_name::<CameraTrackingSystem>(),
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

    fn setup_small_map(world: &mut World) {
        let map_bounds = MapBounds::new(0, 0, 0, 400, 300, 200);
        setup_map_selection(world, map_bounds, "test/small_map")
    }

    fn setup_big_map(world: &mut World) {
        let map_bounds = MapBounds::new(
            0,
            0,
            0,
            MAP_WIDTH as u32,
            MAP_HEIGHT as u32,
            MAP_DEPTH as u32,
        );
        setup_map_selection(world, map_bounds, "test/big_map")
    }

    fn setup_system_data(world: &mut World) {
        <CameraTrackingSystem as System<'_>>::SystemData::setup(world);
        <Read<'_, AssetIdMappings> as SystemData<'_>>::setup(world);
    }

    fn setup_map_selection(world: &mut World, map_bounds: MapBounds, slug: &str) {
        let map_selection = {
            let map_margins = Margins::from(map_bounds);

            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            let mut asset_map_bounds = world.write_resource::<AssetMapBounds>();
            let mut asset_margins = world.write_resource::<AssetMargins>();
            let slug = AssetSlug::from_str(slug).expect("Expected asset slug to be valid.");

            let asset_id = asset_id_mappings.insert(slug);
            asset_map_bounds.insert(asset_id, map_bounds);
            asset_margins.insert(asset_id, map_margins);

            MapSelection::Id(asset_id)
        };

        world.insert(map_selection);
    }

    struct SetupParams {
        position_mirroreds: Vec<(Position<f32>, Option<Mirrored>)>,
        setup_map_selection_fn: fn(&mut World),
    }

    struct ExpectedParams {
        camera_target_coordinates: CameraTargetCoordinates,
    }
}

#[cfg(test)]
mod tests {
    use std::{any, str::FromStr};

    use amethyst::{
        ecs::{Builder, Entity, Read, System, SystemData, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use enumflags2::BitFlags;
    use kinematic_model::config::Position;
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::MapBounds,
        loaded::{AssetMargins, Margins},
        play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
    };
    use map_selection_model::MapSelection;
    use tracker::Last;

    use map_play::MapEnterExitDetectionSystem;

    #[test]
    fn does_not_send_event_when_remaining_in_map() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: None,
            },
        )
    }

    #[test]
    fn does_not_send_event_when_remaining_out_of_map() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(-1., 200., 0.),
                position: Position::new(-1., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: None,
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_left() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(-1., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Left);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_right() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(800., 200., 0.),
                position: Position::new(801., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Right);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(0., 199., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Bottom);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_top() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 800., 0.),
                position: Position::new(0., 801., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Top);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_back() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(0., 200., -1.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Back);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_front() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 200.),
                position: Position::new(0., 200., 201.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Front);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_left_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(-1., 199., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Left | BoundaryFace::Bottom;
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_right_top() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(800., 800., 0.),
                position: Position::new(801., 801., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Right | BoundaryFace::Top;
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_bottom_back() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(0., 199., -1.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Bottom | BoundaryFace::Back;
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_top_front() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 800., 200.),
                position: Position::new(0., 801., 201.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Top | BoundaryFace::Front;
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_exit_event_when_exiting_map_left_bottom_back() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 0.),
                position: Position::new(-1., 199., -1.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces =
                        BoundaryFace::Left | BoundaryFace::Bottom | BoundaryFace::Back;
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_left() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(-1., 200., 0.),
                position: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Left);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_right() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(801., 200., 0.),
                position: Position::new(800., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Right);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 199., 0.),
                position: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Bottom);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_top() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 801., 0.),
                position: Position::new(0., 800., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Top);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_back() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., -1.),
                position: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Back);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_front() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 200., 201.),
                position: Position::new(0., 200., 200.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Front);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_left_top() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(-1., 801., 0.),
                position: Position::new(0., 800., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Left | BoundaryFace::Top;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_right_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(801., 199., 0.),
                position: Position::new(800., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Right | BoundaryFace::Bottom;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_bottom_front() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 199., 201.),
                position: Position::new(0., 200., 200.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Bottom | BoundaryFace::Front;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_top_back() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(0., 801., -1.),
                position: Position::new(0., 800., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BoundaryFace::Top | BoundaryFace::Back;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    #[test]
    fn sends_enter_event_when_entering_map_right_top_front() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_last: Position::new(801., 801., 201.),
                position: Position::new(800., 800., 200.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces =
                        BoundaryFace::Right | BoundaryFace::Top | BoundaryFace::Front;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    fn run_test(
        SetupParams {
            position_last,
            position,
        }: SetupParams,
        ExpectedParams {
            map_boundary_event_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(MapLoadingBundle::new())
            .with_effect(setup_system_data)
            .with_effect(setup_map_selection)
            .with_effect(setup_event_reader)
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(Last::new(position_last))
                    .with(position)
                    .build();

                world.insert(entity);
            })
            .with_system_single(
                MapEnterExitDetectionSystem::new(),
                any::type_name::<MapEnterExitDetectionSystem>(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let map_boundary_event_rid =
                    &mut *world.write_resource::<ReaderId<MapBoundaryEvent>>();
                let map_boundary_ec = world.read_resource::<EventChannel<MapBoundaryEvent>>();
                let events_actual = map_boundary_ec
                    .read(map_boundary_event_rid)
                    .copied()
                    .collect::<Vec<MapBoundaryEvent>>();

                if let Some(map_boundary_event_fn) = map_boundary_event_fn {
                    let entity = *world.read_resource::<Entity>();
                    let map_boundary_event = map_boundary_event_fn(entity);

                    assert_eq!(vec![map_boundary_event], events_actual);
                } else {
                    assert!(events_actual.is_empty());
                }
            })
            .run()
    }

    fn setup_map_selection(world: &mut World) {
        let map_selection = {
            let map_bounds = MapBounds::new(0, 0, 0, 800, 600, 200);
            let map_margins = Margins::from(map_bounds);

            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            let mut asset_margins = world.write_resource::<AssetMargins>();
            let slug =
                AssetSlug::from_str("test/empty_map").expect("Expected asset slug to be valid.");

            let asset_id = asset_id_mappings.insert(slug);
            asset_margins.insert(asset_id, map_margins);

            MapSelection::Id(asset_id)
        };

        world.insert(map_selection);
    }

    fn setup_system_data(world: &mut World) {
        <MapEnterExitDetectionSystem as System<'_>>::SystemData::setup(world);
        <Read<'_, AssetIdMappings> as SystemData>::setup(world);
    }

    fn setup_event_reader(world: &mut World) {
        let map_boundary_event_rid = world
            .write_resource::<EventChannel<MapBoundaryEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(map_boundary_event_rid);
    }

    struct SetupParams {
        position_last: Position<f32>,
        position: Position<f32>,
    }

    struct ExpectedParams {
        map_boundary_event_fn: Option<fn(Entity) -> MapBoundaryEvent>,
    }
}

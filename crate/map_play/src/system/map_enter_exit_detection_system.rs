use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use enumflags2::BitFlags;
use kinematic_model::config::Position;
use map_model::{
    loaded::{Map, Margins},
    play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
};
use map_selection_model::MapSelection;
use tracker::Last;
use typename_derive::TypeName;

/// Sends a `MapBoundaryEvent` when an entity's `Position` has entered or exited map bounds.
#[derive(Debug, Default, TypeName, new)]
pub struct MapEnterExitDetectionSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapEnterExitDetectionSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub maps: Read<'s, AssetStorage<Map>>,
    /// `Last<Position<f32>>` components.
    #[derivative(Debug = "ignore")]
    pub positions_last: ReadStorage<'s, Last<Position<f32>>>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Write<'s, EventChannel<MapBoundaryEvent>>,
}

impl<'s> System<'s> for MapEnterExitDetectionSystem {
    type SystemData = MapEnterExitDetectionSystemData<'s>;

    fn run(
        &mut self,
        MapEnterExitDetectionSystemData {
            entities,
            map_selection,
            maps,
            positions_last,
            positions,
            mut map_boundary_ec,
        }: Self::SystemData,
    ) {
        let map_margins = {
            maps.get(map_selection.handle())
                .map(|map| &map.margins)
                .expect("Expected map to be loaded.")
        };

        // Send event when the entity was in bounds previously, but not in bounds now.
        let map_boundary_events = (&entities, &positions_last, &positions)
            .join()
            .filter_map(|(entity, position_last, position)| {
                Self::detect_enter_exit(map_margins, entity, **position_last, *position)
            })
            .collect::<Vec<MapBoundaryEvent>>();
        map_boundary_ec.iter_write(map_boundary_events);
    }
}

/// Where a value lies in comparison to a range.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Comparative {
    /// Below the range.
    Below,
    /// Within the range.
    Within,
    /// Above the range.
    Above,
}

impl MapEnterExitDetectionSystem {
    /// Returns a `MapBoundaryEvent` if the entity has crossed the map boundary.
    fn detect_enter_exit(
        map_margins: &Margins,
        entity: Entity,
        position_last: Position<f32>,
        position: Position<f32>,
    ) -> Option<MapBoundaryEvent> {
        let (within_x_last, within_y_last, within_z_last) =
            Self::position_comparative(map_margins, position_last);
        let within_bounds_last =
            Self::is_within_bounds(within_x_last, within_y_last, within_z_last);

        let (within_x, within_y, within_z) = Self::position_comparative(map_margins, position);
        let within_bounds = Self::is_within_bounds(within_x, within_y, within_z);

        let mut boundary_faces = BitFlags::<BoundaryFace>::default();

        if within_bounds_last && !within_bounds {
            match within_x {
                Comparative::Below => boundary_faces |= BoundaryFace::Left,
                Comparative::Above => boundary_faces |= BoundaryFace::Right,
                Comparative::Within => {}
            }
            match within_y {
                Comparative::Below => boundary_faces |= BoundaryFace::Bottom,
                Comparative::Above => boundary_faces |= BoundaryFace::Top,
                Comparative::Within => {}
            }
            match within_z {
                Comparative::Below => boundary_faces |= BoundaryFace::Back,
                Comparative::Above => boundary_faces |= BoundaryFace::Front,
                Comparative::Within => {}
            }
            Some(MapBoundaryEvent::Exit(MapBoundaryEventData {
                entity,
                boundary_faces,
            }))
        } else if !within_bounds_last && within_bounds {
            match within_x_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Left,
                Comparative::Above => boundary_faces |= BoundaryFace::Right,
                Comparative::Within => {}
            }
            match within_y_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Bottom,
                Comparative::Above => boundary_faces |= BoundaryFace::Top,
                Comparative::Within => {}
            }
            match within_z_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Back,
                Comparative::Above => boundary_faces |= BoundaryFace::Front,
                Comparative::Within => {}
            }

            Some(MapBoundaryEvent::Enter(MapBoundaryEventData {
                entity,
                boundary_faces,
            }))
        } else {
            None
        }
    }

    /// Returns whether the position is within the map margins.
    fn is_within_bounds(
        within_x: Comparative,
        within_y: Comparative,
        within_z: Comparative,
    ) -> bool {
        within_x == Comparative::Within
            && within_y == Comparative::Within
            && within_z == Comparative::Within
    }

    /// Returns a 3-tuple of `Comparative`s whether the position is within margins on each axis.
    fn position_comparative(
        map_margins: &Margins,
        position: Position<f32>,
    ) -> (Comparative, Comparative, Comparative) {
        let within_x = Self::value_comparative(map_margins.left, map_margins.right, position[0]);
        let within_y = Self::value_comparative(map_margins.bottom, map_margins.top, position[1]);
        let within_z = Self::value_comparative(map_margins.back, map_margins.front, position[2]);

        (within_x, within_y, within_z)
    }

    /// Returns whether the value is between the lower and upper limits (inclusive at both ends).
    fn value_comparative(lower: f32, upper: f32, value: f32) -> Comparative {
        if value >= lower {
            if value <= upper {
                Comparative::Within
            } else {
                Comparative::Above
            }
        } else {
            Comparative::Below
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, System, SystemData, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
    use enumflags2::BitFlags;
    use kinematic_model::config::Position;
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::{MapBounds, MapDefinition, MapHeader},
        loaded::{Map, Margins},
        play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
    };
    use map_selection_model::MapSelection;
    use tracker::Last;
    use typename::TypeName;

    use super::MapEnterExitDetectionSystem;

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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                let map_handle = {
                    let map = empty_map();
                    let loader = world.read_resource::<Loader>();
                    let map_assets = world.read_resource::<AssetStorage<Map>>();

                    loader.load_from_data(map, (), &map_assets)
                };

                let slug = AssetSlug::from_str("test/empty_map")
                    .expect("Expected asset slug to be valid.");
                let snh = SlugAndHandle::new(slug, map_handle);
                let map_selection = MapSelection::Id(snh);

                world.insert(map_selection);
            })
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let entity = world
                    .create_entity()
                    .with(Last::new(position_last))
                    .with(position)
                    .build();

                world.insert(entity);
            })
            .with_system_single(
                MapEnterExitDetectionSystem::new(),
                MapEnterExitDetectionSystem::type_name(),
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

    fn empty_map() -> Map {
        let map_bounds = MapBounds::new(0, 0, 0, 800, 600, 200);
        let map_header = MapHeader::new(String::from("empty_map"), map_bounds);
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
        <MapEnterExitDetectionSystem as System<'_>>::SystemData::setup(world);
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

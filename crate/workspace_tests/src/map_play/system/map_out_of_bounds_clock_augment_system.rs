#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use enumflags2::BitFlags;
    use map_model::play::{
        BoundaryFace, MapBoundaryEvent, MapBoundaryEventData, MapUnboundedDelete,
        OutOfBoundsDeleteClock,
    };
    use std::any;

    use map_play::{MapOutOfBoundsClockAugmentSystem, OUT_OF_BOUNDS_DELETE_DELAY};

    #[test]
    fn does_not_change_out_of_bounds_delete_clock_when_no_map_boundary_event() -> Result<(), Error>
    {
        let out_of_bounds_delete_clock = OutOfBoundsDeleteClock::new_with_value(10, 5);
        run_test(
            SetupParams {
                out_of_bounds_delete_clock: Some(out_of_bounds_delete_clock),
                map_boundary_event_fn: None,
            },
            ExpectedParams {
                out_of_bounds_delete_clock: Some(out_of_bounds_delete_clock),
            },
        )
    }

    #[test]
    fn augments_out_of_bounds_delete_clock_on_exit_event() -> Result<(), Error> {
        let out_of_bounds_delete_clock = OutOfBoundsDeleteClock::new(OUT_OF_BOUNDS_DELETE_DELAY);
        run_test(
            SetupParams {
                out_of_bounds_delete_clock: None,
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Left);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
            ExpectedParams {
                out_of_bounds_delete_clock: Some(out_of_bounds_delete_clock),
            },
        )
    }

    #[test]
    fn removes_out_of_bounds_delete_clock_on_enter_event() -> Result<(), Error> {
        let out_of_bounds_delete_clock = OutOfBoundsDeleteClock::new_with_value(10, 5);
        run_test(
            SetupParams {
                out_of_bounds_delete_clock: Some(out_of_bounds_delete_clock),
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Left);
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
            ExpectedParams {
                out_of_bounds_delete_clock: None,
            },
        )
    }

    fn run_test(
        SetupParams {
            out_of_bounds_delete_clock,
            map_boundary_event_fn,
        }: SetupParams,
        ExpectedParams {
            out_of_bounds_delete_clock: out_of_bounds_delete_clock_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                MapOutOfBoundsClockAugmentSystem::new(),
                any::type_name::<MapOutOfBoundsClockAugmentSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let entity = {
                    let mut entity_builder = world.create_entity().with(MapUnboundedDelete);

                    if let Some(out_of_bounds_delete_clock) = out_of_bounds_delete_clock {
                        entity_builder = entity_builder.with(out_of_bounds_delete_clock);
                    }

                    entity_builder.build()
                };

                if let Some(map_boundary_event_fn) = map_boundary_event_fn {
                    let map_boundary_event = map_boundary_event_fn(entity);
                    let mut map_boundary_ec =
                        world.write_resource::<EventChannel<MapBoundaryEvent>>();

                    map_boundary_ec.single_write(map_boundary_event);
                }

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let out_of_bounds_delete_clocks = world.read_storage::<OutOfBoundsDeleteClock>();
                let out_of_bounds_delete_clock_actual =
                    out_of_bounds_delete_clocks.get(entity).copied();

                assert_eq!(
                    out_of_bounds_delete_clock_expected,
                    out_of_bounds_delete_clock_actual
                );
            })
            .run()
    }

    struct SetupParams {
        out_of_bounds_delete_clock: Option<OutOfBoundsDeleteClock>,
        map_boundary_event_fn: Option<fn(Entity) -> MapBoundaryEvent>,
    }

    struct ExpectedParams {
        out_of_bounds_delete_clock: Option<OutOfBoundsDeleteClock>,
    }
}

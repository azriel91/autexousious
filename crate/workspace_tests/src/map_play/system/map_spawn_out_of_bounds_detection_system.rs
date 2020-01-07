#[cfg(test)]
mod tests {
    use std::{any, str::FromStr};

    use amethyst::{
        ecs::{Builder, Entity, Read, System, SystemData, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application_test_support::AssetQueries;
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use enumflags2::BitFlags;
    use kinematic_model::config::{Position, Velocity};
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::MapBounds,
        loaded::{AssetMargins, Margins},
        play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
    };
    use map_selection_model::MapSelection;
    use sequence_model::loaded::SequenceId;
    use spawn_model::{loaded::Spawn, play::SpawnEvent};

    use map_play::MapSpawnOutOfBoundsDetectionSystem;

    #[test]
    fn does_not_send_event_when_spawned_in_map_bounds() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_spawn: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: None,
            },
        )
    }

    #[test]
    fn sends_exit_event_when_spawned_out_of_map_bounds() -> Result<(), Error> {
        run_test(
            SetupParams {
                position_spawn: Position::new(-1., 200., 0.),
            },
            ExpectedParams {
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::<BoundaryFace>::default();
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
        )
    }

    fn run_test(
        SetupParams { position_spawn }: SetupParams,
        ExpectedParams {
            map_boundary_event_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(setup_system_data)
            .with_setup(setup_map_selection)
            .with_setup(setup_event_reader)
            .with_bundle(MapLoadingBundle::new())
            .with_system(
                MapSpawnOutOfBoundsDetectionSystem::new(),
                any::type_name::<MapSpawnOutOfBoundsDetectionSystem>(),
                &[],
            )
            .with_effect(move |world| {
                let entity_spawned = world.create_entity().with(position_spawn).build();

                world.insert(entity_spawned);

                send_spawn_event(world, entity_spawned);
            })
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
        <MapSpawnOutOfBoundsDetectionSystem as System<'_>>::SystemData::setup(world);
        <Read<'_, AssetIdMappings> as SystemData>::setup(world);
    }

    fn setup_event_reader(world: &mut World) {
        let map_boundary_event_rid = world
            .write_resource::<EventChannel<MapBoundaryEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(map_boundary_event_rid);
    }

    fn send_spawn_event(world: &mut World, entity_spawned: Entity) {
        let asset_id = AssetQueries::id_generate_any(world);
        let spawn = Spawn::new(
            asset_id,
            Position::default(),
            Velocity::default(),
            SequenceId::new(0),
        );
        let entity_parent = world.create_entity().build();
        let spawn_event = SpawnEvent::new(spawn, entity_parent, entity_spawned, asset_id);

        let mut spawn_ec = world.write_resource::<EventChannel<SpawnEvent>>();
        spawn_ec.single_write(spawn_event);
    }

    struct SetupParams {
        position_spawn: Position<f32>,
    }

    struct ExpectedParams {
        map_boundary_event_fn: Option<fn(Entity) -> MapBoundaryEvent>,
    }
}

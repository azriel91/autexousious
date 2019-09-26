use amethyst::{
    ecs::{Read, System, World, Write, WriteExpect},
    shred::SystemData,
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use derive_new::new;
use map_selection_model::{MapSelection, MapSelectionEvent};
use typename_derive::TypeName;

use crate::MapSelectionStatus;

/// Updates the `MapSelection` resource based on user selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSystem {
    /// ID for reading map selection events.
    #[new(default)]
    map_selection_event_rid: Option<ReaderId<MapSelectionEvent>>,
}

type MapSelectionSystemData<'s, 'c> = (
    Write<'s, MapSelectionStatus>,
    Read<'s, EventChannel<MapSelectionEvent>>,
    WriteExpect<'s, MapSelection>,
);

impl<'s> System<'s> for MapSelectionSystem {
    type SystemData = MapSelectionSystemData<'s, 's>;

    fn run(
        &mut self,
        (mut map_selection_status, selection_event_channel, mut map_selection): Self::SystemData,
    ) {
        selection_event_channel
            .read(
                self.map_selection_event_rid
                    .as_mut()
                    .expect("Expected `map_selection_event_rid` to be set."),
            )
            .for_each(|ev| match ev {
                MapSelectionEvent::Return => {}
                MapSelectionEvent::Switch {
                    map_selection: selection,
                }
                | MapSelectionEvent::Select {
                    map_selection: selection,
                } => {
                    *map_selection = *selection;
                }
                MapSelectionEvent::Deselect => {
                    *map_selection_status = MapSelectionStatus::Pending;
                }
                MapSelectionEvent::Confirm => {
                    *map_selection_status = MapSelectionStatus::Confirmed;
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        if world.try_fetch::<MapSelection>().is_none() {
            let first_map_id = world
                .fetch::<AssetTypeMappings>()
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.");

            world.insert(MapSelection::Random(Some(first_map_id)));
        }

        let mut selection_event_channel = world.fetch_mut::<EventChannel<MapSelectionEvent>>();
        self.map_selection_event_rid = Some(selection_event_channel.register_reader());
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use amethyst::{
        ecs::{Read, SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    };
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use typename::TypeName;

    use super::{MapSelectionSystem, MapSelectionSystemData};
    use crate::MapSelectionStatus;

    #[test]
    fn does_nothing_on_return() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Pending,
                map_selection_event_fn: |_world| MapSelectionEvent::Return,
            },
            ExpectedParams {
                map_select: MapSelect::One,
                map_selection_status: MapSelectionStatus::Pending,
            },
        )
    }

    #[test]
    fn sets_map_selection_on_switch() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Pending,
                map_selection_event_fn: |world| {
                    let map_selection = map_selection(world, MapSelect::Two);
                    MapSelectionEvent::Switch { map_selection }
                },
            },
            ExpectedParams {
                map_select: MapSelect::Two,
                map_selection_status: MapSelectionStatus::Pending,
            },
        )
    }

    #[test]
    fn sets_map_selection_on_select() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Pending,
                map_selection_event_fn: |world| {
                    let map_selection = map_selection(world, MapSelect::Two);
                    MapSelectionEvent::Select { map_selection }
                },
            },
            ExpectedParams {
                map_select: MapSelect::Two,
                map_selection_status: MapSelectionStatus::Pending,
            },
        )
    }

    #[test]
    fn pending_map_selection_status_on_deselect() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Confirmed,
                map_selection_event_fn: |_world| MapSelectionEvent::Deselect,
            },
            ExpectedParams {
                map_select: MapSelect::One,
                map_selection_status: MapSelectionStatus::Pending,
            },
        )
    }

    #[test]
    fn confirms_map_selection_status_on_confirm() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Pending,
                map_selection_event_fn: |_world| MapSelectionEvent::Confirm,
            },
            ExpectedParams {
                map_select: MapSelect::One,
                map_selection_status: MapSelectionStatus::Confirmed,
            },
        )
    }

    fn run_test(
        SetupParams {
            map_selection_status: map_selection_status_setup,
            map_selection_event_fn,
        }: SetupParams,
        ExpectedParams {
            map_selection_status: map_selection_status_expected,
            map_select: map_select_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(map_selection_status_setup)
            .with_system(
                MapSelectionSystem::new(),
                MapSelectionSystem::type_name(),
                &[],
            )
            .with_setup(setup_maps)
            .with_setup(setup_system_data)
            .with_effect(move |world| {
                let initial_selection = {
                    let map_asset_ids = &*world.read_resource::<Vec<AssetId>>();
                    MapSelection::Id(map_asset_ids[0])
                };
                world.insert(initial_selection);

                let map_selection_event = map_selection_event_fn(world);
                send_event(world, map_selection_event)
            })
            .with_assertion(move |world| {
                let map_selection_expected = map_selection(world, map_select_expected);
                let map_selection_actual = world.read_resource::<MapSelection>();
                assert_eq!(map_selection_expected, *map_selection_actual);

                let map_selection_status = world.read_resource::<MapSelectionStatus>();
                assert_eq!(map_selection_status_expected, *map_selection_status);
            })
            .run()
    }

    fn setup_system_data(world: &mut World) {
        MapSelectionSystemData::setup(world);
    }

    fn setup_maps(world: &mut World) {
        <Read<'_, AssetIdMappings> as SystemData>::setup(world);
        <Read<'_, AssetTypeMappings> as SystemData>::setup(world);

        let asset_ids = {
            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            let mut asset_type_mappings = world.write_resource::<AssetTypeMappings>();
            let slug_map_one =
                AssetSlug::from_str("test/map_one").expect("Expected asset slug to be valid.");
            let slug_map_two =
                AssetSlug::from_str("test/map_two").expect("Expected asset slug to be valid.");

            let asset_id_one = asset_id_mappings.insert(slug_map_one);
            let asset_id_two = asset_id_mappings.insert(slug_map_two);

            asset_type_mappings.insert(asset_id_one, AssetType::Map);
            asset_type_mappings.insert(asset_id_two, AssetType::Map);

            vec![asset_id_one, asset_id_two]
        };

        world.insert(asset_ids);
    }

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        world
            .write_resource::<EventChannel<MapSelectionEvent>>()
            .single_write(event);
    }

    fn map_selection(world: &World, map_select: MapSelect) -> MapSelection {
        let index = match map_select {
            MapSelect::One => 0,
            MapSelect::Two => 1,
        };
        let map_asset_ids = &*world.read_resource::<Vec<AssetId>>();
        MapSelection::Id(map_asset_ids[index])
    }

    struct SetupParams {
        map_selection_status: MapSelectionStatus,
        map_selection_event_fn: fn(&mut World) -> MapSelectionEvent,
    }

    struct ExpectedParams {
        map_select: MapSelect,
        map_selection_status: MapSelectionStatus,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum MapSelect {
        One,
        Two,
    }
}

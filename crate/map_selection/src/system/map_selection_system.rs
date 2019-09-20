use amethyst::{
    ecs::{Read, System, World, Write, WriteExpect},
    shred::SystemData,
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use derive_new::new;
use log::warn;
use map_selection_model::{MapSelection, MapSelectionEvent};
use typename_derive::TypeName;

use crate::MapSelectionStatus;

/// Updates the `MapSelection` resource based on user selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSystem {
    /// ID for reading map selection events.
    #[new(default)]
    reader_id: Option<ReaderId<MapSelectionEvent>>,
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
        if let MapSelectionStatus::Confirmed = *map_selection_status {
            return;
        }

        let mut events = selection_event_channel.read(self.reader_id.as_mut().unwrap());

        if let Some(MapSelectionEvent::Select {
            map_selection: selection,
        }) = events.next()
        {
            *map_selection_status = MapSelectionStatus::Confirmed;
            *map_selection = *selection;

            // Discard additional events, and log a message
            let additional_events = events.count();
            if additional_events > 0 {
                warn!(
                    "Discarding `{}` additional map selection events.",
                    additional_events
                );
            }
        }
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
        self.reader_id = Some(selection_event_channel.register_reader());
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use amethyst::{
        ecs::{SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::AssetSlug,
        loaded::{AssetId, AssetIdMappings},
    };
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use typename::TypeName;

    use super::{MapSelectionSystem, MapSelectionSystemData};
    use crate::MapSelectionStatus;

    #[test]
    fn returns_when_map_selection_status_confirmed() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Confirmed,
                map_select: MapSelect::Two,
            },
            ExpectedParams {
                map_selection_status: MapSelectionStatus::Confirmed,
                map_select: MapSelect::One,
            },
        )
    }

    #[test]
    fn selects_map_when_select_event_is_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                map_selection_status: MapSelectionStatus::Pending,
                map_select: MapSelect::Two,
            },
            ExpectedParams {
                map_selection_status: MapSelectionStatus::Confirmed,
                map_select: MapSelect::Two,
            },
        )
    }

    fn run_test(
        SetupParams {
            map_selection_status: map_selection_status_setup,
            map_select: map_select_setup,
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

                // Send event, if the event is not responded to, then we know the system returns
                // early.
                let map_selection = {
                    let index = match map_select_setup {
                        MapSelect::One => 0,
                        MapSelect::Two => 1,
                    };
                    let map_asset_ids = &*world.read_resource::<Vec<AssetId>>();
                    MapSelection::Id(map_asset_ids[index])
                };

                send_event(world, MapSelectionEvent::Select { map_selection })
            })
            .with_assertion(move |world| {
                let map_selection_expected = {
                    let index = match map_select_expected {
                        MapSelect::One => 0,
                        MapSelect::Two => 1,
                    };
                    let map_asset_ids = &*world.read_resource::<Vec<AssetId>>();
                    MapSelection::Id(map_asset_ids[index])
                };
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
        let asset_ids = {
            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            let slug_map_one =
                AssetSlug::from_str("test/map_one").expect("Expected asset slug to be valid.");
            let slug_map_two =
                AssetSlug::from_str("test/map_two").expect("Expected asset slug to be valid.");

            let asset_id_one = asset_id_mappings.insert(slug_map_one);
            let asset_id_two = asset_id_mappings.insert(slug_map_two);

            vec![asset_id_one, asset_id_two]
        };

        world.insert(asset_ids);
    }

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        world
            .write_resource::<EventChannel<MapSelectionEvent>>()
            .single_write(event);
    }

    struct SetupParams {
        map_selection_status: MapSelectionStatus,
        map_select: MapSelect,
    }

    struct ExpectedParams {
        map_selection_status: MapSelectionStatus,
        map_select: MapSelect,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum MapSelect {
        One,
        Two,
    }
}

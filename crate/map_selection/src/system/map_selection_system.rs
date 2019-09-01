use amethyst::{ecs::prelude::*, shrev::EventChannel};

use asset_model::loaded::SlugAndHandle;
use derive_new::new;
use game_model::loaded::MapPrefabs;
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
            *map_selection = selection.clone();

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
            let slug_and_handle = world
                .fetch::<MapPrefabs>()
                .iter()
                .next()
                .map(SlugAndHandle::from)
                .expect("Expected at least one map to be loaded.");

            world.insert(MapSelection::Random(slug_and_handle));
        }

        let mut selection_event_channel = world.fetch_mut::<EventChannel<MapSelectionEvent>>();
        self.reader_id = Some(selection_event_channel.register_reader());
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::ProgressCounter,
        core::TransformBundle,
        ecs::SystemData,
        prelude::*,
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shrev::EventChannel,
    };
    use amethyst_test::AmethystApplication;
    use asset_loading::AssetDiscovery;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::{ASSETS_PATH, MAP_EMPTY_SLUG, MAP_FADE_SLUG};
    use loading::AssetLoader;
    use map_loading::MapLoadingBundle;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;

    use super::{MapSelectionSystem, MapSelectionSystemData};
    use crate::MapSelectionStatus;

    #[test]
    fn returns_when_map_selection_status_confirmed() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
                .with_bundle(SpriteLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_resource(MapSelectionStatus::Confirmed)
                .with_effect(setup_components)
                .with_effect(load_maps)
                .with_effect(|world| {
                    let fade_snh = SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()));
                    let map_selection = MapSelection::Id(fade_snh);
                    world.insert(map_selection);

                    // Send event, if the event is not responded to, then we know the system returns
                    // early.
                    let empty_snh = SlugAndHandle::from((&*world, MAP_EMPTY_SLUG.clone()));
                    let map_selection = MapSelection::Id(empty_snh);
                    send_event(world, MapSelectionEvent::Select { map_selection })
                })
                .with_system_single(
                    MapSelectionSystem::new(),
                    MapSelectionSystem::type_name(),
                    &[],
                )
                .with_assertion(|world| {
                    let fade_snh = SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()));

                    let map_selection = world.read_resource::<MapSelection>();
                    assert_eq!(MapSelection::Id(fade_snh), *map_selection);
                })
                .run_isolated()
                .is_ok()
        );
    }

    #[test]
    #[ignore]
    // TODO: Fails because the reader ID is registered after the event is sent.
    // See <https://gitlab.com/azriel91/autexousious/issues/74>
    fn selects_map_when_select_event_is_sent() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
                .with_bundle(SpriteLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_effect(setup_components)
                .with_effect(load_maps)
                .with_effect(|world| {
                    let fade_snh = SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()));
                    let map_selection = MapSelection::Id(fade_snh);
                    world.insert(map_selection);

                    // Send event, if the event is responded to, then we know the system has read
                    // it.
                    let empty_snh = SlugAndHandle::from((&*world, MAP_EMPTY_SLUG.clone()));
                    let map_selection = MapSelection::Id(empty_snh);
                    send_event(world, MapSelectionEvent::Select { map_selection })
                })
                .with_system_single(
                    MapSelectionSystem::new(),
                    MapSelectionSystem::type_name(),
                    &[],
                )
                .with_assertion(|world| {
                    let empty_snh = SlugAndHandle::from((&*world, MAP_EMPTY_SLUG.clone()));

                    let map_selection = world.read_resource::<MapSelection>();
                    assert_eq!(MapSelection::Id(empty_snh), *map_selection);

                    let map_selection_status = world.read_resource::<MapSelectionStatus>();
                    assert_eq!(MapSelectionStatus::Confirmed, *map_selection_status);
                })
                .run_isolated()
                .is_ok()
        );
    }

    fn setup_components(world: &mut World) {
        MapSelectionSystemData::setup(world);
    }

    fn load_maps(world: &mut World) {
        let asset_index = AssetDiscovery::asset_index(&ASSETS_PATH);

        let mut progress_counter = ProgressCounter::new();
        AssetLoader::load_maps(world, &mut progress_counter, asset_index.maps);
    }

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        world
            .write_resource::<EventChannel<MapSelectionEvent>>()
            .single_write(event);
    }
}

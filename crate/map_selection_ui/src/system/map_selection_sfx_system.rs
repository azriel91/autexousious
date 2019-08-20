use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, System, SystemData, World},
    shrev::{EventChannel, ReaderId},
};
use derive_new::new;
use map_selection_model::MapSelectionEvent;
use typename_derive::TypeName;
use ui_audio_model::{config::UiSfxId, loaded::UiSfxMap};

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays sounds for the map selection UI.
#[derive(Debug, Default, TypeName, new)]
pub struct MapSelectionSfxSystem {
    /// Reader ID for the `MapSelectionEvent` event channel.
    #[new(default)]
    map_selection_event_rid: Option<ReaderId<MapSelectionEvent>>,
}

type MapSelectionSfxSystemData<'s> = (
    Read<'s, EventChannel<MapSelectionEvent>>,
    Read<'s, UiSfxMap>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
);

impl<'s> System<'s> for MapSelectionSfxSystem {
    type SystemData = MapSelectionSfxSystemData<'s>;

    fn run(&mut self, (map_selection_ec, ui_sfx_map, source_assets, output): Self::SystemData) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = map_selection_ec.read(
            self.map_selection_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for MapSelectionSfxSystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| {
                let ui_sfx_id = match ev {
                    MapSelectionEvent::Return => UiSfxId::Cancel,
                    MapSelectionEvent::Switch { .. } => UiSfxId::Switch,
                    MapSelectionEvent::Select { .. } => UiSfxId::Select,
                    MapSelectionEvent::Deselect { .. } => UiSfxId::Deselect,
                    MapSelectionEvent::Confirm => UiSfxId::Confirm,
                };

                let ui_sfx = ui_sfx_map
                    .get(&ui_sfx_id)
                    .and_then(|ui_sfx_handle| source_assets.get(ui_sfx_handle));

                if let Some(ui_sfx) = ui_sfx {
                    output.play_once(ui_sfx, VOLUME);
                }
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.map_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<MapSelectionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::MAP_FADE_SLUG;
    use map_selection_model::{MapSelection, MapSelectionEvent};

    use super::MapSelectionSfxSystem;

    #[test]
    fn plays_sound_on_return_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Return)
    }

    #[test]
    fn plays_sound_on_switch_event() -> Result<(), Error> {
        run_test(|world| {
            let snh = SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()));
            let map_selection = MapSelection::Id(snh);
            MapSelectionEvent::Switch { map_selection }
        })
    }

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        run_test(|world| {
            let snh = SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()));
            let map_selection = MapSelection::Id(snh);
            MapSelectionEvent::Select { map_selection }
        })
    }

    #[test]
    fn plays_sound_on_deselect_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Deselect)
    }

    #[test]
    fn plays_sound_on_confirm_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Confirm)
    }

    fn run_test<F>(event_fn: F) -> Result<(), Error>
    where
        F: Fn(&mut World) -> MapSelectionEvent + Send + Sync + 'static,
    {
        AutexousiousApplication::config_base()
            .with_system(MapSelectionSfxSystem::new(), "", &[])
            .with_effect(move |world| {
                let event = event_fn(world);
                send_event(world, event);
            })
            .with_assertion(|_world| {})
            .run_isolated()
    }

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<MapSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}

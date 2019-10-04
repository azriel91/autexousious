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

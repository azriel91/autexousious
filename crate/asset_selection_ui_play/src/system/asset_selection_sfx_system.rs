use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::play::AssetSelectionEvent;
use derivative::Derivative;
use derive_new::new;
use ui_audio_model::{config::UiSfxId, loaded::UiSfxMap};

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays sounds for the character selection UI.
#[derive(Debug, Default, new)]
pub struct AssetSelectionSfxSystem {
    /// Reader ID for the `AssetSelectionEvent` event channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

/// `AssetSelectionSfxSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionSfxSystemData<'s> {
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
    /// `UiSfxMap` resource.
    #[derivative(Debug = "ignore")]
    pub ui_sfx_map: Read<'s, UiSfxMap>,
    /// `Source` assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `Output` resource.
    #[derivative(Debug = "ignore")]
    pub output: Option<Read<'s, Output>>,
}

impl<'s> System<'s> for AssetSelectionSfxSystem {
    type SystemData = AssetSelectionSfxSystemData<'s>;

    fn run(
        &mut self,
        AssetSelectionSfxSystemData {
            asset_selection_ec,
            ui_sfx_map,
            source_assets,
            output,
        }: Self::SystemData,
    ) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = asset_selection_ec.read(
            self.asset_selection_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for AssetSelectionSfxSystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| {
                let ui_sfx_id = match ev {
                    AssetSelectionEvent::Return => UiSfxId::Cancel,
                    AssetSelectionEvent::Join { .. } => UiSfxId::Select,
                    AssetSelectionEvent::Switch { .. } => UiSfxId::Switch,
                    AssetSelectionEvent::Select { .. } => UiSfxId::Select,
                    AssetSelectionEvent::Deselect { .. } => UiSfxId::Deselect,
                    AssetSelectionEvent::Leave { .. } => UiSfxId::Deselect,
                    AssetSelectionEvent::Confirm => UiSfxId::Confirm,
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
        self.asset_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<AssetSelectionEvent>>()
                .register_reader(),
        );
    }
}

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::CharacterSelectionEvent;
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;
use ui_audio_model::{config::UiSfxId, loaded::UiSfxMap};

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays sounds for the character selection UI.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionSfxSystem {
    /// Reader ID for the `CharacterSelectionEvent` event channel.
    #[new(default)]
    character_selection_event_rid: Option<ReaderId<CharacterSelectionEvent>>,
}

/// `CharacterSelectionSfxSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionSfxSystemData<'s> {
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Read<'s, EventChannel<CharacterSelectionEvent>>,
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

impl<'s> System<'s> for CharacterSelectionSfxSystem {
    type SystemData = CharacterSelectionSfxSystemData<'s>;

    fn run(
        &mut self,
        CharacterSelectionSfxSystemData {
            character_selection_ec,
            ui_sfx_map,
            source_assets,
            output,
        }: Self::SystemData,
    ) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = character_selection_ec.read(
            self.character_selection_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for CharacterSelectionSfxSystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| {
                let ui_sfx_id = match ev {
                    CharacterSelectionEvent::Return => UiSfxId::Cancel,
                    CharacterSelectionEvent::Join { .. } => UiSfxId::Select,
                    CharacterSelectionEvent::Switch { .. } => UiSfxId::Switch,
                    CharacterSelectionEvent::Select { .. } => UiSfxId::Select,
                    CharacterSelectionEvent::Deselect { .. } => UiSfxId::Deselect,
                    CharacterSelectionEvent::Leave { .. } => UiSfxId::Deselect,
                    CharacterSelectionEvent::Confirm => UiSfxId::Confirm,
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
        self.character_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}

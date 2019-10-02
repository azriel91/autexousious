use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, ReadStorage, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use audio_model::loaded::SourceHandleOpt;
use derivative::Derivative;
use derive_new::new;
use sequence_model::play::SequenceUpdateEvent;
use typename_derive::TypeName;

/// Default volume to play sounds at.
const VOLUME: f32 = 1.0;

/// Plays a sound at the beginning of a frame.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceAudioPlaySystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceAudioPlaySystemData<'s> {
    /// `SequenceUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SourceHandleOpt` components.
    #[derivative(Debug = "ignore")]
    pub source_handle_opts: ReadStorage<'s, SourceHandleOpt>,
    /// `Source` assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `Output` resource.
    #[derivative(Debug = "ignore")]
    pub output: Option<Read<'s, Output>>,
}

impl<'s> System<'s> for SequenceAudioPlaySystem {
    type SystemData = SequenceAudioPlaySystemData<'s>;

    fn run(
        &mut self,
        SequenceAudioPlaySystemData {
            sequence_update_ec,
            source_handle_opts,
            source_assets,
            output,
        }: Self::SystemData,
    ) {
        // Make sure we empty the event channel, even if we don't have an output device.
        let events_iterator = sequence_update_ec.read(
            self.sequence_update_event_rid
                .as_mut()
                .expect("Expected reader ID to exist for SequenceAudioPlaySystem."),
        );

        if let Some(output) = output {
            events_iterator.for_each(|ev| match ev {
                SequenceUpdateEvent::SequenceBegin { entity, .. }
                | SequenceUpdateEvent::FrameBegin { entity, .. } => {
                    let source_handle_opt = source_handle_opts.get(*entity);

                    let source = source_handle_opt
                        .and_then(|source_handle_opt| source_handle_opt.as_ref())
                        .and_then(|source_handle| source_assets.get(source_handle));

                    if let Some(source) = source {
                        output.play_once(source, VOLUME);
                    }
                }
                SequenceUpdateEvent::SequenceEnd { .. } => {}
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

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
            events_iterator.for_each(|ev| {
                let source_handle_opt = source_handle_opts.get(ev.entity());

                let source = source_handle_opt
                    .and_then(|source_handle_opt| source_handle_opt.as_ref())
                    .and_then(|source_handle| source_assets.get(source_handle));

                if let Some(source) = source {
                    output.play_once(source, VOLUME);
                }
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

#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        assets::{AssetStorage, Loader, ProgressCounter},
        audio::{AudioBundle, Source},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
        shrev::EventChannel,
        Error, State, StateData, Trans,
    };
    use amethyst_test::{AmethystApplication, GameUpdate};
    use audio_loading::{AudioLoader, AudioLoadingBundle};
    use audio_model::loaded::SourceHandleOpt;
    use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};

    use super::SequenceAudioPlaySystem;

    #[test]
    fn plays_sound_on_sequence_update_event() -> Result<(), Error> {
        run_test(|entity| SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SequenceId::new(0),
        })
    }

    fn run_test(sequence_update_event_fn: fn(Entity) -> SequenceUpdateEvent) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(AudioBundle::default())
            .with_system(SequenceAudioPlaySystem::new(), "", &[])
            .with_effect(|world| {
                let mut progress_counter = ProgressCounter::new();

                let source_handle_opt = {
                    let (loader, source_assets) = world
                        .system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Source>>)>();
                    let source_handle = AudioLoader::load(
                        &loader,
                        &source_assets,
                        &mut progress_counter,
                        &empty_wav_path(),
                    );
                    SourceHandleOpt::new(Some(source_handle))
                };

                world.insert(progress_counter);
                world.insert(source_handle_opt);
            })
            .with_state(|| WaitForLoad)
            .with_effect(move |world| {
                let source_handle_opt = (*world.read_resource::<SourceHandleOpt>()).clone();
                let entity = world.create_entity().with(source_handle_opt).build();

                let event = sequence_update_event_fn(entity);
                send_event(world, event);
            })
            .with_assertion(|_world| {})
            .run()
    }

    fn empty_wav_path() -> PathBuf {
        PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "assets",
            "test",
            "sfx",
            "empty.wav",
        ])
    }

    fn send_event(world: &mut World, event: SequenceUpdateEvent) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    #[derive(Debug)]
    struct WaitForLoad;
    impl<T, E> State<T, E> for WaitForLoad
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let progress_counter = data.world.read_resource::<ProgressCounter>();
            if !progress_counter.is_complete() {
                Trans::None // kcov-ignore
            } else {
                Trans::Pop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use amethyst::{
        assets::{AssetStorage, Loader, ProgressCounter},
        audio::{AudioBundle, Source},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::{AmethystApplication, WaitForLoad};
    use audio_loading::{AudioLoader, AudioLoadingBundle};
    use audio_model::loaded::SourceHandleOpt;
    use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};

    use audio_play::SequenceAudioPlaySystem;

    #[test]
    fn plays_sound_on_sequence_begin_event() -> Result<(), Error> {
        run_test(|entity| SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SequenceId::new(0),
        })
    }

    #[test]
    fn plays_sound_on_frame_begin_event() -> Result<(), Error> {
        run_test(|entity| SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index: 0,
        })
    }

    #[test]
    fn does_not_play_sound_on_sequence_end_event() -> Result<(), Error> {
        run_test(|entity| SequenceUpdateEvent::SequenceEnd {
            entity,
            frame_index: 1,
        })
    }

    fn run_test(sequence_update_event_fn: fn(Entity) -> SequenceUpdateEvent) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(AudioBundle::default())
            .with_system(SequenceAudioPlaySystem::new(), "", &[])
            .with_effect(|world| {
                let mut progress_counter = ProgressCounter::new();
                let path = Path::new("test/sfx/empty.wav");

                let source_handle_opt = {
                    let (loader, source_assets) = world
                        .system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Source>>)>();
                    let source_handle =
                        AudioLoader::load(&loader, &source_assets, &mut progress_counter, path);
                    SourceHandleOpt::new(Some(source_handle))
                };

                world.insert(progress_counter);
                world.insert(source_handle_opt);
            })
            .with_state(WaitForLoad::new)
            .with_effect(move |world| {
                let source_handle_opt = (*world.read_resource::<SourceHandleOpt>()).clone();
                let entity = world.create_entity().with(source_handle_opt).build();

                let event = sequence_update_event_fn(entity);
                send_event(world, event);
            })
            .with_assertion(|_world| {
                // TODO: assert that sound was played / not played
            })
            .run()
    }

    fn send_event(world: &mut World, event: SequenceUpdateEvent) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}

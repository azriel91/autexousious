use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::{
    CharacterControlTransitionsHandle, CharacterCts, CharacterCtsHandle,
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};

/// Updates the `CharacterControlTransitionsHandle` when sequence ID changes.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterControlTransitionsUpdateSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterControlTransitionsUpdateSystemData<'s> {
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `CharacterCtsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_cts_handles: ReadStorage<'s, CharacterCtsHandle>,
    /// `CharacterCts` assets.
    #[derivative(Debug = "ignore")]
    pub character_cts_assets: Read<'s, AssetStorage<CharacterCts>>,
    /// `CharacterControlTransitionsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_handles: WriteStorage<'s, CharacterControlTransitionsHandle>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub character_sequence_names: ReadStorage<'s, SequenceId>,
}

impl<'s> System<'s> for CharacterControlTransitionsUpdateSystem {
    type SystemData = CharacterControlTransitionsUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterControlTransitionsUpdateSystemData {
            sequence_update_ec,
            character_cts_handles,
            character_cts_assets,
            mut character_control_transitions_handles,
            character_sequence_names,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id.as_mut().expect(
                    "Expected reader ID to exist for CharacterControlTransitionsUpdateSystem.",
                ),
            )
            // kcov-ignore-start
            .filter(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { .. }
                | SequenceUpdateEvent::FrameBegin { .. } = ev
                {
                    true
                } else {
                    false
                }
            })
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                // `SequenceUpdateEvent`s are also sent for non-object entities such as map layers
                if let Some(character_cts_handle) = character_cts_handles.get(entity) {
                    let character_cts = character_cts_assets
                        .get(character_cts_handle)
                        .expect("Expected `CharacterCts` to be loaded.");

                    if frame_index < character_cts.len() {
                        let character_control_transitions_handle = &character_cts[frame_index];

                        character_control_transitions_handles
                            .insert(entity, character_control_transitions_handle.clone())
                            .expect("Failed to insert `CharacterControlTransitions` component.");
                    } else {
                        let character_sequence_name = character_sequence_names.get(entity).expect(
                            "Expected entity with `CharacterCtsHandle` \
                             to have `SequenceId`.",
                        );

                        error!(
                            "Attempted to access index `{}` for sequence ID: `{:?}`",
                            frame_index, character_sequence_name
                        );
                    }
                }
            });
        // kcov-ignore-end
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

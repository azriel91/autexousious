use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::{CharacterInputReactionsHandle, CharacterIrs, CharacterIrsHandle};
use derivative::Derivative;
use derive_new::new;
use log::error;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};

/// Updates the `CharacterInputReactionsHandle` when sequence ID changes.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterInputReactionsUpdateSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterInputReactionsUpdateSystemData<'s> {
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `CharacterIrsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_irs_handles: ReadStorage<'s, CharacterIrsHandle>,
    /// `CharacterIrs` assets.
    #[derivative(Debug = "ignore")]
    pub character_irs_assets: Read<'s, AssetStorage<CharacterIrs>>,
    /// `CharacterInputReactionsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_input_reactions_handles: WriteStorage<'s, CharacterInputReactionsHandle>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub character_sequence_names: ReadStorage<'s, SequenceId>,
}

impl<'s> System<'s> for CharacterInputReactionsUpdateSystem {
    type SystemData = CharacterInputReactionsUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterInputReactionsUpdateSystemData {
            sequence_update_ec,
            character_irs_handles,
            character_irs_assets,
            mut character_input_reactions_handles,
            character_sequence_names,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterInputReactionsUpdateSystem."),
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
                if let Some(character_irs_handle) = character_irs_handles.get(entity) {
                    let character_irs = character_irs_assets
                        .get(character_irs_handle)
                        .expect("Expected `CharacterIrs` to be loaded.");

                    if frame_index < character_irs.len() {
                        let character_input_reactions_handle = &character_irs[frame_index];

                        character_input_reactions_handles
                            .insert(entity, character_input_reactions_handle.clone())
                            .expect("Failed to insert `CharacterInputReactions` component.");
                    } else {
                        let character_sequence_name = character_sequence_names.get(entity).expect(
                            "Expected entity with `CharacterIrsHandle` \
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

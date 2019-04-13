use amethyst::{
    assets::AssetStorage,
    ecs::{
        storage::ComponentEvent, BitSet, Join, Read, ReadStorage, ReaderId, System, SystemData,
        WriteStorage,
    },
    shred::Resources,
};
use character_model::{
    config::CharacterSequenceId,
    loaded::{Character, CharacterControlTransitionsSequenceHandle, CharacterHandle},
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use named_type::NamedType;
use named_type_derive::NamedType;
use shred_derive::SystemData;

/// Updates the attached `CharacterControlTransitionsSequenceHandle`s when `SequenceId` changes.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterCtsHandleUpdateSystem {
    /// Reader ID for sequence ID changes.
    #[new(default)]
    sequence_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `CharacterSequenceId`s.
    #[new(default)]
    sequence_id_updates: BitSet,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterCtsHandleUpdateSystemData<'s> {
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, CharacterSequenceId>,
    /// `CharacterHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_handles: ReadStorage<'s, CharacterHandle>,
    /// `Character` assets.
    #[derivative(Debug = "ignore")]
    pub character_assets: Read<'s, AssetStorage<Character>>,
    /// `CharacterControlTransitionsSequenceHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_cts_handles: WriteStorage<'s, CharacterControlTransitionsSequenceHandle>,
}

impl<'s> System<'s> for CharacterCtsHandleUpdateSystem {
    type SystemData = CharacterCtsHandleUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterCtsHandleUpdateSystemData {
            sequence_ids,
            character_handles,
            character_assets,
            mut character_cts_handles,
        }: Self::SystemData,
    ) {
        self.sequence_id_updates.clear();

        sequence_ids
            .channel()
            .read(
                self.sequence_id_rid
                    .as_mut()
                    .expect("Expected `sequence_id_rid` to be set."),
            )
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.sequence_id_updates.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        (
            &sequence_ids,
            &character_handles,
            &mut character_cts_handles,
            &self.sequence_id_updates,
        )
            .join()
            .for_each(
                |(sequence_id, character_handle, character_cts_handle, _)| {
                    let character = character_assets
                        .get(&character_handle)
                        .expect("Expected `ObjectWrapper` to be loaded.");
                    let character_cts_handles =
                        &character.control_transitions_sequence_handles;

                    *character_cts_handle = character_cts_handles
                        .get(&sequence_id)
                        .unwrap_or_else(|| {
                            let message = format!(
                                "Expected `CharacterControlTransitionsSequenceHandle` to exist for\
                                 sequence ID: `{:?}`. Falling back to default sequence.",
                                sequence_id
                            );
                            error!("{}", message);

                            let default_sequence_id = CharacterSequenceId::default();

                            character_cts_handles
                                .get(&default_sequence_id)
                                .unwrap_or_else(|| {
                                    let message = format!(
                                        "Failed to get `CharacterControlTransitionsSequenceHandle` for \
                                         sequence ID: `{:?}`.",
                                        default_sequence_id
                                    );
                                    error!("{}", message);
                                    panic!(message);
                                })
                        })
                        .clone()
                },
            );
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.sequence_id_rid =
            Some(WriteStorage::<CharacterSequenceId>::fetch(&res).register_reader());
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        Error,
    };
    use application_test_support::{AutexousiousApplication, ObjectQueries, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_loading::CharacterPrefab;
    use character_model::{
        config::CharacterSequenceId, loaded::CharacterControlTransitionsSequenceHandle,
    };

    use super::CharacterCtsHandleUpdateSystem;

    #[test]
    fn attaches_handle_for_sequence_id_insertions() -> Result<(), Error> {
        let test_name = "attaches_handle_for_sequence_id_insertions";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(CharacterCtsHandleUpdateSystem::new(), "", &[])
            .with_setup(|world| insert_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_component_sequences(world, CharacterSequenceId::RunStop))
            .run()
    }

    #[test]
    fn attaches_handle_for_sequence_id_modifications() -> Result<(), Error> {
        let test_name = "attaches_handle_for_sequence_id_modifications";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(CharacterCtsHandleUpdateSystem::new(), "", &[])
            .with_setup(|world| update_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_component_sequences(world, CharacterSequenceId::RunStop))
            .run()
    }

    fn insert_sequence(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = create_entity(world);

        {
            let mut sequence_ids = world.write_storage::<CharacterSequenceId>();
            sequence_ids
                .insert(entity, sequence_id)
                .expect("Failed to insert `CharacterSequenceId`.");
        }

        world.add_resource(entity);
    }

    fn update_sequence(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = create_entity(world);

        {
            let mut sequence_ids = world.write_storage::<CharacterSequenceId>();
            let sid = sequence_ids
                .get_mut(entity)
                .expect("Expected entity to contain `CharacterSequenceId` component.");
            *sid = sequence_id;
        }

        world.add_resource(entity);
    }

    fn create_entity(world: &mut World) -> Entity {
        let asset_slug = ASSETS_CHAR_BAT_SLUG.clone();
        let character_handle =
            ObjectQueries::game_object_handle::<CharacterPrefab>(world, &asset_slug)
                .expect("Expected `CharacterHandle` to exist.");
        let character_cts_handle =
            SequenceQueries::character_cts_handle(world, &asset_slug, CharacterSequenceId::Stand);

        world
            .create_entity()
            .with(character_handle)
            .with(CharacterSequenceId::Stand)
            .with(character_cts_handle)
            .build()
    }

    fn expect_component_sequences(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = *world.read_resource::<Entity>();
        let expected_handle = SequenceQueries::character_cts_handle(
            world,
            &ASSETS_CHAR_BAT_SLUG.clone(),
            sequence_id,
        );
        let character_cts_handles =
            world.read_storage::<CharacterControlTransitionsSequenceHandle>();

        assert_eq!(
            &expected_handle,
            character_cts_handles.get(entity).expect(
                "Expected entity to contain `CharacterControlTransitionsSequenceHandle` component."
            )
        );
    }
}

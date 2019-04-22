use amethyst::{assets::AssetStorage, ecs::World};
use asset_model::config::AssetSlug;
use character_loading::CharacterPrefab;
use character_model::{
    config::CharacterSequenceId,
    loaded::{
        Character, CharacterControlTransitionsHandle, CharacterControlTransitionsSequence,
        CharacterControlTransitionsSequenceHandle, CharacterObjectWrapper,
    },
};
use object_model::loaded::ObjectWrapper;
use sequence_model::loaded::ComponentSequencesHandle;

use crate::ObjectQueries;

/// Functions to retrieve sequence data from a running world.
#[derive(Debug)]
pub struct SequenceQueries;

impl SequenceQueries {
    /// Returns the `CharacterControlTransitionsSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to retrieve.
    /// * `sequence_id`: Sequence ID whose `CharacterControlTransitionsSequenceHandle` to retrieve.
    pub fn character_cts_handle(
        world: &mut World,
        asset_slug: &AssetSlug,
        sequence_id: CharacterSequenceId,
    ) -> CharacterControlTransitionsSequenceHandle {
        let character_handle = ObjectQueries::game_object_handle::<CharacterPrefab>(
            world, asset_slug,
        )
        .unwrap_or_else(|| {
            panic!(
                "Expected `CharacterPrefab` to exist for slug: `{}`.",
                asset_slug
            )
        });
        let character_assets = world.read_resource::<AssetStorage<Character>>();
        let character = character_assets.get(&character_handle).unwrap_or_else(|| {
            panic!(
                "Expected `Character` to be loaded for slug: `{}`.",
                asset_slug
            )
        });

        character
            .control_transitions_sequence_handles
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `CharacterControlTransitionsSequenceHandle` to exist for sequence \
                     ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `CharacterControlTransitionsHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to retrieve.
    /// * `sequence_id`: Sequence ID whose `CharacterControlTransitionsSequenceHandle` to retrieve.
    /// * `frame_index`: Frame index within the sequence whose control transitions to retrieve.
    pub fn character_control_transitions_handle(
        world: &mut World,
        asset_slug: &AssetSlug,
        sequence_id: CharacterSequenceId,
        frame_index: usize,
    ) -> CharacterControlTransitionsHandle {
        let character_cts_handle = Self::character_cts_handle(world, asset_slug, sequence_id);

        let character_cts_assets =
            world.read_resource::<AssetStorage<CharacterControlTransitionsSequence>>();
        let character_cts = character_cts_assets
            .get(&character_cts_handle)
            .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

        character_cts[frame_index].clone()
    }

    /// Returns the `ComponentSequencesHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to retrieve.
    /// * `sequence_id`: Sequence ID whose `ComponentSequencesHandle` to retrieve.
    pub fn component_sequences_handle(
        world: &mut World,
        asset_slug: &AssetSlug,
        sequence_id: CharacterSequenceId,
    ) -> ComponentSequencesHandle {
        let object_wrapper_handle = ObjectQueries::object_wrapper_handle(world, asset_slug);
        let object_wrapper_assets = world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
        let object_wrapper = object_wrapper_assets
            .get(&object_wrapper_handle)
            .unwrap_or_else(|| panic!("Expected `{}` object wrapper to be loaded.", asset_slug));

        object_wrapper
            .inner()
            .component_sequences_handles
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected component_sequences_handles for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }
}

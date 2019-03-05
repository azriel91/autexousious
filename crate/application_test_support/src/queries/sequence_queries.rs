use amethyst::{assets::AssetStorage, ecs::World};
use asset_model::config::AssetSlug;
use character_model::{config::CharacterSequenceId, loaded::CharacterObjectWrapper};
use object_model::loaded::ObjectWrapper;
use sequence_model::loaded::ComponentSequencesHandle;

use crate::ObjectQueries;

/// Functions to retrieve sequence data from a running world.
#[derive(Debug)]
pub struct SequenceQueries;

impl SequenceQueries {
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
        let object_wrapper_handle = ObjectQueries::object_wrapper_handle(world, &asset_slug);
        let object_wrapper_assets = world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
        let object_wrapper = object_wrapper_assets
            .get(&object_wrapper_handle)
            .unwrap_or_else(|| panic!("Expected `{}` object wrapper to be loaded.", &asset_slug));

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

use amethyst::{
    assets::AssetStorage,
    ecs::{World, WorldExt},
};
use asset_model::config::AssetSlug;
use character_model::loaded::{
    AssetCharacterCtsHandles, CharacterControlTransitionsHandle, CharacterCts, CharacterCtsHandle,
};
use collision_model::loaded::{
    AssetBodySequenceHandles, AssetInteractionsSequenceHandles, BodySequenceHandle,
    InteractionsSequenceHandle,
};
use sequence_model::loaded::{
    AssetSequenceEndTransitions, AssetWaitSequenceHandles, SequenceEndTransition, SequenceId,
    WaitSequenceHandle,
};
use spawn_model::loaded::{AssetSpawnsSequenceHandles, SpawnsSequenceHandle};
use sprite_model::loaded::{AssetSpriteRenderSequenceHandles, SpriteRenderSequenceHandle};

use crate::AssetQueries;

/// Functions to retrieve sequence data from a running world.
#[derive(Debug)]
pub struct SequenceQueries;

impl SequenceQueries {
    /// Returns the `CharacterCtsHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to retrieve.
    /// * `sequence_id`: Sequence ID whose `CharacterCtsHandle` to retrieve.
    pub fn character_cts_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> CharacterCtsHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_character_cts_handles = world.read_resource::<AssetCharacterCtsHandles>();
        let character_cts_handles =
            asset_character_cts_handles
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `CharacterCtsHandles` to exist for `{:?}`.",
                        asset_id
                    )
                });

        character_cts_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `CharacterCtsHandle` to exist for sequence ID: `{:?}`.",
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
    /// * `sequence_id`: Sequence ID whose `CharacterCtsHandle` to retrieve.
    /// * `frame_index`: Frame index within the sequence whose control transitions to retrieve.
    pub fn character_control_transitions_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
        frame_index: usize,
    ) -> CharacterControlTransitionsHandle {
        let character_cts_handle = Self::character_cts_handle(world, asset_slug, sequence_id);

        let character_cts_assets = world.read_resource::<AssetStorage<CharacterCts>>();
        let character_cts = character_cts_assets
            .get(&character_cts_handle)
            .expect("Expected `CharacterCts` to be loaded.");

        character_cts[frame_index].clone()
    }

    /// Returns the `SequenceEndTransition` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `SequenceEndTransition`.
    pub fn sequence_end_transition(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> SequenceEndTransition {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_sequence_end_transitions = world.read_resource::<AssetSequenceEndTransitions>();
        let sequence_end_transitions =
            asset_sequence_end_transitions
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceEndTransitions` to exist for `{:?}`.",
                        asset_id
                    )
                });

        sequence_end_transitions
            .get(*sequence_id)
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceEndTransition` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
    }

    /// Returns the `WaitSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `WaitSequenceHandle`.
    pub fn wait_sequence_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> WaitSequenceHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_wait_sequence_handles = world.read_resource::<AssetWaitSequenceHandles>();
        let wait_sequence_handles =
            asset_wait_sequence_handles
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `WaitSequenceHandles` to exist for `{:?}`.",
                        asset_id
                    )
                });

        wait_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `WaitSequenceHandle` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `SpriteRenderSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `SpriteRenderSequenceHandle`.
    pub fn sprite_render_sequence_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> SpriteRenderSequenceHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_sprite_render_handles = world.read_resource::<AssetSpriteRenderSequenceHandles>();
        let sprite_render_handles =
            asset_sprite_render_handles
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SpriteRenderSequenceHandles` to exist for `{:?}`.",
                        asset_id
                    )
                });

        sprite_render_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpriteRenderSequenceHandle` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `BodySequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `BodySequenceHandle`.
    pub fn body_sequence_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> BodySequenceHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_body_sequence_handles = world.read_resource::<AssetBodySequenceHandles>();
        let body_sequence_handles =
            asset_body_sequence_handles
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `BodySequenceHandles` to exist for `{:?}`.",
                        asset_id
                    )
                });

        body_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `BodySequenceHandle` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `InteractionsSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `InteractionsSequenceHandle`.
    pub fn interactions_sequence_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> InteractionsSequenceHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_interactions_sequence_handles =
            world.read_resource::<AssetInteractionsSequenceHandles>();
        let interactions_sequence_handles = asset_interactions_sequence_handles
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `InteractionsSequenceHandles` to exist for `{:?}`.",
                    asset_id
                )
            });

        interactions_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `InteractionsSequenceHandle` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `SpawnsSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Asset slug of the `Object`.
    /// * `sequence_id`: Sequence ID of the `SpawnsSequenceHandle`.
    pub fn spawns_sequence_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> SpawnsSequenceHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_spawns_sequence_handles = world.read_resource::<AssetSpawnsSequenceHandles>();
        let spawns_sequence_handles =
            asset_spawns_sequence_handles
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SpawnsSequenceHandles` to exist for `{:?}`.",
                        asset_id
                    )
                });

        spawns_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpawnsSequenceHandle` for sequence ID `{:?}` to exist.",
                    sequence_id
                )
            })
            .clone()
    }
}

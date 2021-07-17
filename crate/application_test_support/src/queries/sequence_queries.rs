use amethyst::{
    assets::AssetStorage,
    ecs::{World, WorldExt},
};
use asset_model::{config::AssetSlug, loaded::AssetItemIds, play::AssetWorld};
use character_model::loaded::{
    CharacterInputReactionsHandle, CharacterIrs, CharacterIrsHandle, CharacterIrsHandles,
};
use collision_model::loaded::{
    BodySequenceHandle, BodySequenceHandles, InteractionsSequenceHandle,
    InteractionsSequenceHandles,
};
use sequence_model::loaded::{
    SequenceEndTransition, SequenceEndTransitions, SequenceId, WaitSequenceHandle,
    WaitSequenceHandles,
};
use spawn_model::loaded::{SpawnsSequenceHandle, SpawnsSequenceHandles};
use sprite_model::loaded::{SpriteRenderSequenceHandle, SpriteRenderSequenceHandles};

use crate::AssetQueries;

/// Functions to retrieve sequence data from a running world.
#[derive(Debug)]
pub struct SequenceQueries;

impl SequenceQueries {
    /// Returns the `CharacterIrsHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to
    ///   retrieve.
    /// * `sequence_id`: Sequence ID whose `CharacterIrsHandle` to retrieve.
    pub fn character_irs_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
    ) -> CharacterIrsHandle {
        let asset_id = AssetQueries::id(world, &asset_slug);
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let character_irs_handles = asset_world
            .read_storage::<CharacterIrsHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `CharacterIrsHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        character_irs_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `CharacterIrsHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `CharacterInputReactionsHandle` for the specified sequence
    /// ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: Object slug whose `Handle<O::ObjectWrapper>` to
    ///   retrieve.
    /// * `sequence_id`: Sequence ID whose `CharacterIrsHandle` to retrieve.
    /// * `frame_index`: Frame index within the sequence whose input reactions
    ///   to retrieve.
    pub fn character_input_reactions_handle(
        world: &World,
        asset_slug: &AssetSlug,
        sequence_id: SequenceId,
        frame_index: usize,
    ) -> CharacterInputReactionsHandle {
        let character_irs_handle = Self::character_irs_handle(world, asset_slug, sequence_id);

        let character_irs_assets = world.read_resource::<AssetStorage<CharacterIrs>>();
        let character_irs = character_irs_assets
            .get(&character_irs_handle)
            .expect("Expected `CharacterIrs` to be loaded.");

        character_irs[frame_index].clone()
    }

    /// Returns the `SequenceEndTransition` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let sequence_end_transitions = asset_world
            .read_storage::<SequenceEndTransitions>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceEndTransitions` to exist for `{:?}`.",
                    asset_slug
                )
            });

        sequence_end_transitions
            .get(*sequence_id)
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceEndTransition` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
    }

    /// Returns the `WaitSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let wait_sequence_handles = asset_world
            .read_storage::<WaitSequenceHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `WaitSequenceHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        wait_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `WaitSequenceHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `SpriteRenderSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let sprite_render_sequence_handles = asset_world
            .read_storage::<SpriteRenderSequenceHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpriteRenderSequenceHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        sprite_render_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpriteRenderSequenceHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `BodySequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let body_sequence_handles = asset_world
            .read_storage::<BodySequenceHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `BodySequenceHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        body_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `BodySequenceHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `InteractionsSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let interactions_sequence_handles = asset_world
            .read_storage::<InteractionsSequenceHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `InteractionsSequenceHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        interactions_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `InteractionsSequenceHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }

    /// Returns the `SpawnsSequenceHandle` for the specified sequence ID.
    ///
    /// This function assumes the character for the specified slug is
    /// instantiated in the world.
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
        let asset_world = world.read_resource::<AssetWorld>();
        let asset_item_ids = world.read_resource::<AssetItemIds>();
        let item_id_first = asset_item_ids
            .get(asset_id)
            .unwrap_or_else(|| panic!("Expected `ItemIds` to exist for `{:?}`.", asset_slug))
            .first()
            .unwrap_or_else(|| panic!("Expected one `ItemId` to exist for `{:?}`.", asset_slug));
        let spawns_sequence_handles = asset_world
            .read_storage::<SpawnsSequenceHandles>()
            .get(item_id_first.0)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpawnsSequenceHandles` to exist for `{:?}`.",
                    asset_slug
                )
            });

        spawns_sequence_handles
            .get(*sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SpawnsSequenceHandle` to exist for sequence ID: `{:?}`.",
                    sequence_id
                )
            })
            .clone()
    }
}

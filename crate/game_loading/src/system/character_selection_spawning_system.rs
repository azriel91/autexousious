use amethyst::{
    ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetItemIds, ItemId};
use character_prefab::{
    CharacterComponentStorages, CharacterEntityAugmenter, CharacterSpawningResources,
};
use character_selection_model::CharacterSelections;
use derivative::Derivative;
use derive_new::new;
use game_input_model::play::InputControlled;
use game_model::play::GameEntities;
use object_type::ObjectType;
use team_model::play::{IndependentCounter, Team};

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, new)]
pub struct CharacterSelectionSpawningSystem;

/// `CharacterSelectionSpawningSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionSpawningSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `CharacterSelections` resource.
    #[derivative(Debug = "ignore")]
    pub character_selections: Read<'s, CharacterSelections>,
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `IndependentCounter` resource.
    #[derivative(Debug = "ignore")]
    pub independent_counter: Write<'s, IndependentCounter>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Read<'s, AssetItemIds>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `CharacterSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub character_spawning_resources: CharacterSpawningResources<'s>,
    /// `CharacterComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub character_component_storages: CharacterComponentStorages<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: WriteStorage<'s, Team>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
}

impl<'s> System<'s> for CharacterSelectionSpawningSystem {
    type SystemData = CharacterSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        CharacterSelectionSpawningSystemData {
            entities,
            character_selections,
            mut game_loading_status,
            mut independent_counter,
            asset_item_ids,
            asset_id_mappings,
            mut asset_ids,
            mut item_ids,
            character_spawning_resources,
            mut character_component_storages,
            mut input_controlleds,
            mut teams,
            mut game_entities,
        }: Self::SystemData,
    ) {
        if game_loading_status.character_augment_status != CharacterAugmentStatus::Prefab {
            return;
        }

        let character_entities = character_selections
            .selections
            .iter()
            .map(|(controller_id, asset_id)| {
                let asset_id = *asset_id;
                let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
                    panic!(
                        "Expected `AssetSlug` to exist for `AssetId`: {:?}",
                        asset_id
                    )
                });
                let item_ids_character = asset_item_ids.get(asset_id).unwrap_or_else(|| {
                    panic!("Expected `ItemIds` to exist for asset: `{}`", asset_slug);
                });
                let item_id = item_ids_character.first().copied().unwrap_or_else(|| {
                    panic!("Expected `ItemId` to exist for asset: `{}`", asset_slug)
                });

                let entity = entities.create();

                CharacterEntityAugmenter::augment(
                    &character_spawning_resources,
                    &mut character_component_storages,
                    asset_id,
                    entity,
                );

                asset_ids
                    .insert(entity, asset_id)
                    .expect("Failed to insert `AssetId` for character.");
                item_ids
                    .insert(entity, item_id)
                    .expect("Failed to insert `ItemId` for character.");
                input_controlleds
                    .insert(entity, InputControlled::new(*controller_id))
                    .expect("Failed to insert `InputControlled` for character.");
                teams
                    .insert(
                        entity,
                        Team::Independent(independent_counter.get_and_increment()),
                    )
                    .expect("Failed to insert `Team` for character.");

                entity
            })
            .collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);

        game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
    }
}

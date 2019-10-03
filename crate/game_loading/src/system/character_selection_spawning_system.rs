use amethyst::{
    ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use character_prefab::{
    CharacterComponentStorages, CharacterEntityAugmenter, CharacterSpawningResources,
};
use character_selection_model::CharacterSelections;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_model::play::GameEntities;
use object_prefab::{ObjectComponentStorages, ObjectEntityAugmenter, ObjectSpawningResources};
use object_type::ObjectType;
use team_model::play::{IndependentCounter, Team};
use typename_derive::TypeName;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
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
    /// `ObjectSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub object_spawning_resources: ObjectSpawningResources<'s>,
    /// `ObjectComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub object_component_storages: ObjectComponentStorages<'s>,
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
            object_spawning_resources,
            mut object_component_storages,
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
                let entity = entities.create();
                ObjectEntityAugmenter::augment(
                    &object_spawning_resources,
                    &mut object_component_storages,
                    *asset_id,
                    entity,
                );
                CharacterEntityAugmenter::augment(
                    &character_spawning_resources,
                    &mut character_component_storages,
                    *asset_id,
                    entity,
                );

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

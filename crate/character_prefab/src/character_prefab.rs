use std::mem;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, ProgressCounter},
    ecs::{Entity, Read, ReadExpect, World, WriteStorage},
    shred::{ResourceId, SystemData},
    Error,
};
use character_loading::{
    CharacterLoader, CharacterLoaderParams, ControlTransitionsSequenceLoaderParams,
};
use character_model::{
    config::{CharacterDefinition, CharacterSequenceName},
    loaded::{
        Character, CharacterControlTransitions, CharacterControlTransitionsSequence,
        CharacterHandle, CharacterHitTransitions,
    },
};
use derivative::Derivative;
use log::debug;
use object_model::config::ObjectAssetData;
use object_prefab::{GameObjectPrefab, ObjectPrefab};
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

use crate::{CharacterComponentStorages, CharacterEntityAugmenter};

/// Loads `CharacterDefinition`s and attaches components to character entities.
#[derive(Clone, Debug, PartialEq, TypeName)]
pub enum CharacterPrefab {
    /// Prefab has not been loaded from the assets.
    Data {
        /// The `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
        /// Handle to the `CharacterDefinition`.
        character_definition_handle: Handle<CharacterDefinition>,
    },
    /// Prefab has been loaded from the assets.
    Loaded {
        /// The loaded `ObjectPrefab`.
        object_prefab: ObjectPrefab<Character>,
        /// The loaded `Character`.
        character_handle: CharacterHandle,
        /// Handle to the `CharacterDefinition`.
        character_definition_handle: Handle<CharacterDefinition>,
    },
    /// Temporary value used during loading.
    Invalid,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterPrefabSystemData<'s> {
    #[derivative(Debug = "ignore")]
    pub object_prefab_system_data: <ObjectPrefab<Character> as PrefabData<'s>>::SystemData,
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `CharacterDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub character_definition_assets: Read<'s, AssetStorage<CharacterDefinition>>,
    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterControlTransitionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_sequence_assets:
        Read<'s, AssetStorage<CharacterControlTransitionsSequence>>,
    /// `Character` assets.
    #[derivative(Debug = "ignore")]
    pub character_assets: Read<'s, AssetStorage<Character>>,
    /// `CharacterHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_handles: WriteStorage<'s, CharacterHandle>,
    /// `CharacterHitTransitions` components.
    #[derivative(Debug = "ignore")]
    pub character_hit_transitionses: WriteStorage<'s, CharacterHitTransitions>,
    /// `CharacterComponentStorages` system data.
    pub character_component_storages: CharacterComponentStorages<'s>,
}

impl CharacterPrefab {
    /// Returns a new `CharacterPrefab`.
    ///
    /// # Parameters
    ///
    /// * `object_asset_data`: Assets needed to load an object.
    pub fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        let character_definition_handle = object_asset_data.game_object_definition_handle.clone();

        CharacterPrefab::Data {
            object_prefab: ObjectPrefab::Data(object_asset_data),
            character_definition_handle,
        }
    }
}

impl<'s> PrefabData<'s> for CharacterPrefab {
    type SystemData = CharacterPrefabSystemData<'s>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        CharacterPrefabSystemData {
            object_prefab_system_data,
            ref character_definition_assets,
            ref mut character_handles,
            ref character_assets,
            ref mut character_hit_transitionses,
            ref mut character_component_storages,
            ..
        }: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity],
    ) -> Result<(), Error> {
        match self {
            CharacterPrefab::Loaded {
                object_prefab,
                character_handle,
                character_definition_handle,
            } => {
                debug!("Augmenting entity: {:?}", entity);

                object_prefab.add_to_entity(
                    entity,
                    object_prefab_system_data,
                    entities,
                    children,
                )?;

                character_handles
                    .insert(entity, character_handle.clone())
                    .expect("Failed to insert `CharacterHandle` component.");

                let character_definition = character_definition_assets
                    .get(character_definition_handle)
                    .expect("Expected `CharacterDefinition` to be loaded.");

                CharacterEntityAugmenter::augment(
                    entity,
                    character_component_storages,
                    character_definition,
                );

                let character = character_assets
                    .get(character_handle)
                    .expect("Expected `Character` to be loaded.");
                let sequence_id_mappings = &character.sequence_id_mappings;
                let low_stun = sequence_id_mappings
                    .id(&CharacterSequenceName::Flinch0)
                    .copied()
                    .unwrap_or(SequenceId(0));
                let mid_stun = sequence_id_mappings
                    .id(&CharacterSequenceName::Flinch1)
                    .copied()
                    .unwrap_or(SequenceId(0));
                let high_stun = sequence_id_mappings
                    .id(&CharacterSequenceName::Dazed)
                    .copied()
                    .unwrap_or(SequenceId(0));
                let falling = sequence_id_mappings
                    .id(&CharacterSequenceName::FallForwardAscend)
                    .copied()
                    .unwrap_or(SequenceId(0));

                let character_hit_transitions = CharacterHitTransitions {
                    low_stun,
                    mid_stun,
                    high_stun,
                    falling,
                };
                character_hit_transitionses
                    .insert(entity, character_hit_transitions)
                    .expect("Failed to insert `CharacterHitTransitions` component.");

                Ok(())
            }
            // kcov-ignore-start
            _ => panic!("Expected self to be in the `CharacterPrefab::Loaded` variant."),
            // kcov-ignore-end
        }
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        CharacterPrefabSystemData {
            object_prefab_system_data,
            loader,
            character_definition_assets,
            character_control_transitions_assets,
            character_control_transitions_sequence_assets,
            character_assets,
            ..
        }: &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (self_, needs_loading_result) = match mem::replace(self, CharacterPrefab::Invalid) {
            CharacterPrefab::Data {
                mut object_prefab,
                character_definition_handle,
            } => {
                object_prefab.load_sub_assets(progress, object_prefab_system_data)?;

                if let ObjectPrefab::Handle(object_wrapper_handle) = &object_prefab {
                    let object_wrapper_handle = object_wrapper_handle.clone();

                    let control_transitions_sequence_loader_params =
                        ControlTransitionsSequenceLoaderParams {
                            loader: &loader,
                            character_control_transitions_assets:
                                &character_control_transitions_assets,
                            character_control_transitions_sequence_assets:
                                &character_control_transitions_sequence_assets,
                        };
                    let character_loader_params = CharacterLoaderParams {
                        control_transitions_sequence_loader_params,
                    };
                    let character_definition = character_definition_assets
                        .get(&character_definition_handle)
                        .expect("Expected `CharacterDefinition` to be loaded.");
                    let character = CharacterLoader::load(
                        character_loader_params,
                        character_definition,
                        object_wrapper_handle,
                    )?;
                    let character_handle =
                        loader.load_from_data(character, progress, character_assets);

                    (
                        CharacterPrefab::Loaded {
                            object_prefab,
                            character_handle,
                            character_definition_handle,
                        },
                        Ok(true),
                    )
                } else {
                    // kcov-ignore-start
                    // Should be `unreachable!()`, but would prefer a good error message.
                    (
                        CharacterPrefab::Data {
                            object_prefab,
                            character_definition_handle,
                        },
                        Err(Error::from_string(String::from(
                            "Expected `object_prefab` to be `Handle` variant.",
                        ))),
                    )
                    // kcov-ignore-end
                }
            }
            // kcov-ignore-start
            value @ CharacterPrefab::Loaded { .. } => (value, Ok(false)),
            CharacterPrefab::Invalid => unreachable!(),
            // kcov-ignore-end
        };
        *self = self_;

        needs_loading_result
    }
}

impl<'s> GameObjectPrefab<'s> for CharacterPrefab {
    type GameObject = Character;

    fn new(object_asset_data: ObjectAssetData<CharacterDefinition>) -> Self {
        CharacterPrefab::new(object_asset_data)
    }

    fn game_object_handle(&self) -> Option<CharacterHandle> {
        if let CharacterPrefab::Loaded {
            character_handle, ..
        } = self
        {
            Some(character_handle.clone())
        } else {
            None
        }
    }

    fn object_prefab(&self) -> &ObjectPrefab<Self::GameObject> {
        match self {
            CharacterPrefab::Data { object_prefab, .. }
            | CharacterPrefab::Loaded { object_prefab, .. } => &object_prefab,
            _ => unreachable!(),
        }
    }
}

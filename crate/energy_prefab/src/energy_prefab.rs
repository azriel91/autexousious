use std::mem;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, ProgressCounter},
    ecs::{Entity, Read, ReadExpect, World, WriteStorage},
    shred::{ResourceId, SystemData},
    Error,
};
use collision_model::loaded::{HitTransition, HittingTransition};
use derivative::Derivative;
use energy_loading::EnergyLoader;
use energy_model::{
    config::EnergyDefinition,
    loaded::{Energy, EnergyHandle},
};
use log::debug;
use object_model::config::ObjectAssetData;
use object_prefab::{GameObjectPrefab, ObjectPrefab};
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

use crate::{EnergyComponentStorages, EnergyEntityAugmenter};

/// Loads `EnergyDefinition`s and attaches components to energy entities.
#[derive(Clone, Debug, PartialEq, TypeName)]
pub enum EnergyPrefab {
    /// Prefab has not been loaded from the assets.
    Data {
        /// The `ObjectPrefab`.
        object_prefab: ObjectPrefab<Energy>,
        /// Handle to the `EnergyDefinition`.
        energy_definition_handle: Handle<EnergyDefinition>,
    },
    /// Prefab has been loaded from the assets.
    Loaded {
        /// The loaded `ObjectPrefab`.
        object_prefab: ObjectPrefab<Energy>,
        /// The loaded `Energy`.
        energy_handle: EnergyHandle,
    },
    /// Temporary value used during loading.
    Invalid,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct EnergyPrefabSystemData<'s> {
    #[derivative(Debug = "ignore")]
    pub object_prefab_system_data: <ObjectPrefab<Energy> as PrefabData<'s>>::SystemData,
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `EnergyDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub energy_definition_assets: Read<'s, AssetStorage<EnergyDefinition>>,
    /// `Energy` assets.
    #[derivative(Debug = "ignore")]
    pub energy_assets: Read<'s, AssetStorage<Energy>>,
    /// `EnergyHandle` components.
    #[derivative(Debug = "ignore")]
    pub energy_handles: WriteStorage<'s, EnergyHandle>,
    /// `HitTransition` components.
    #[derivative(Debug = "ignore")]
    pub hit_transitions: WriteStorage<'s, HitTransition>,
    /// `HittingTransition` components.
    #[derivative(Debug = "ignore")]
    pub hitting_transitions: WriteStorage<'s, HittingTransition>,
    /// `EnergyComponentStorages` system data.
    pub energy_component_storages: EnergyComponentStorages<'s>,
}

impl EnergyPrefab {
    /// Returns a new `EnergyPrefab`.
    ///
    /// # Parameters
    ///
    /// * `object_asset_data`: Assets needed to load an object.
    pub fn new(object_asset_data: ObjectAssetData<EnergyDefinition>) -> Self {
        let energy_definition_handle = object_asset_data.game_object_definition_handle.clone();

        EnergyPrefab::Data {
            object_prefab: ObjectPrefab::Data(object_asset_data),
            energy_definition_handle,
        }
    }
}

impl<'s> PrefabData<'s> for EnergyPrefab {
    type SystemData = EnergyPrefabSystemData<'s>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        EnergyPrefabSystemData {
            object_prefab_system_data,
            ref mut energy_handles,
            ref mut energy_component_storages,
            ref mut hit_transitions,
            ref mut hitting_transitions,
            ..
        }: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity],
    ) -> Result<(), Error> {
        match self {
            EnergyPrefab::Loaded {
                object_prefab,
                energy_handle,
            } => {
                debug!("Augmenting entity: {:?}", entity);

                object_prefab.add_to_entity(
                    entity,
                    object_prefab_system_data,
                    entities,
                    children,
                )?;

                energy_handles
                    .insert(entity, energy_handle.clone())
                    .expect("Failed to insert `EnergyHandle` component.");

                EnergyEntityAugmenter::augment(entity, energy_component_storages);

                // Hack: this should be read off an asset.
                hit_transitions
                    .insert(entity, HitTransition::new(SequenceId::new(2)))
                    .expect("Failed to insert `HitTransition` component.");
                hitting_transitions
                    .insert(entity, HittingTransition::new(SequenceId::new(2)))
                    .expect("Failed to insert `HittingTransition` component.");
                // End Hack.

                Ok(())
            }
            // kcov-ignore-start
            _ => panic!("Expected self to be in the `EnergyPrefab::Loaded` variant."),
            // kcov-ignore-end
        }
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        EnergyPrefabSystemData {
            object_prefab_system_data,
            loader,
            energy_definition_assets,
            energy_assets,
            ..
        }: &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (self_, needs_loading_result) = match mem::replace(self, EnergyPrefab::Invalid) {
            EnergyPrefab::Data {
                mut object_prefab,
                energy_definition_handle,
            } => {
                object_prefab.load_sub_assets(progress, object_prefab_system_data)?;

                if let ObjectPrefab::Handle(object_wrapper_handle) = &object_prefab {
                    let object_wrapper_handle = object_wrapper_handle.clone();

                    let energy_definition = energy_definition_assets
                        .get(&energy_definition_handle)
                        .expect("Expected `EnergyDefinition` to be loaded.");
                    let energy = EnergyLoader::load(energy_definition, object_wrapper_handle)?;
                    let energy_handle = loader.load_from_data(energy, progress, energy_assets);

                    (
                        EnergyPrefab::Loaded {
                            object_prefab,
                            energy_handle,
                        },
                        Ok(true),
                    )
                } else {
                    // kcov-ignore-start
                    // Should be `unreachable!()`, but would prefer a good error message.
                    (
                        EnergyPrefab::Data {
                            object_prefab,
                            energy_definition_handle,
                        },
                        Err(Error::from_string(String::from(
                            "Expected `object_prefab` to be `Handle` variant.",
                        ))),
                    )
                    // kcov-ignore-end
                }
            }
            // kcov-ignore-start
            value @ EnergyPrefab::Loaded { .. } => (value, Ok(false)),
            EnergyPrefab::Invalid => unreachable!(),
            // kcov-ignore-end
        };
        *self = self_;

        needs_loading_result
    }
}

impl<'s> GameObjectPrefab<'s> for EnergyPrefab {
    type GameObject = Energy;

    fn new(object_asset_data: ObjectAssetData<EnergyDefinition>) -> Self {
        EnergyPrefab::new(object_asset_data)
    }

    fn game_object_handle(&self) -> Option<EnergyHandle> {
        if let EnergyPrefab::Loaded { energy_handle, .. } = self {
            Some(energy_handle.clone())
        } else {
            None
        }
    }

    fn object_prefab(&self) -> &ObjectPrefab<Self::GameObject> {
        match self {
            EnergyPrefab::Data { object_prefab, .. }
            | EnergyPrefab::Loaded { object_prefab, .. } => &object_prefab,
            _ => unreachable!(),
        }
    }
}

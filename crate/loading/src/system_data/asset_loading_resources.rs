use std::{collections::HashMap, path::PathBuf};

use amethyst::{
    assets::{Loader, ProgressCounter},
    ecs::{Read, ReadExpect, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetTypeMappings};
use derivative::Derivative;
use loading_model::loaded::LoadStatus;
use slotmap::SecondaryMap;

/// `AssetLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingResources<'s> {
    /// `SecondaryMap<AssetId, PathBuf>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_to_path: Read<'s, SecondaryMap<AssetId, PathBuf>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `HashMap<LoadStatus, WaitSequenceHandles>` resource.
    #[derivative(Debug = "ignore")]
    pub load_status_progress_counters: Write<'s, HashMap<LoadStatus, ProgressCounter>>,
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
}

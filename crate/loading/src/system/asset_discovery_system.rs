use std::path::PathBuf;

use amethyst::{
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::AssetDiscovery;
use asset_model::{
    config::AssetIndex,
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStage, LoadStage};
use log::debug;
use slotmap::SecondaryMap;
use typename_derive::TypeName;

/// Discovers assets and writes to `Option<AssetIndex>`.
#[derive(Debug, Default, TypeName, new)]
pub struct AssetDiscoverySystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

/// `AssetDiscoverySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDiscoverySystemData<'s> {
    /// `Option<AssetIndex>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_index: Write<'s, Option<AssetIndex>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Write<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Write<'s, AssetTypeMappings>,
    /// `AssetLoadStage` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Write<'s, AssetLoadStage>,
    /// `SecondaryMap<AssetId, PathBuf>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_to_path: Write<'s, SecondaryMap<AssetId, PathBuf>>,
}

impl<'s> System<'s> for AssetDiscoverySystem {
    type SystemData = AssetDiscoverySystemData<'s>;

    fn run(
        &mut self,
        AssetDiscoverySystemData {
            mut asset_index,
            mut asset_id_mappings,
            mut asset_type_mappings,
            mut asset_load_stage,
            mut asset_id_to_path,
        }: Self::SystemData,
    ) {
        // TODO: Do a diff between existing index and directory based on a file watch / notify.
        // TODO: See <https://github.com/polachok/derive-diff>
        if asset_index.is_none() {
            let asset_index_discovered = AssetDiscovery::asset_index(&self.assets_dir);
            debug!("Indexed assets: {:?}", &asset_index_discovered);

            let capacity = asset_index_discovered
                .values()
                .fold(0, |acc, asset_records| acc + asset_records.len());
            asset_id_mappings.reserve(capacity);

            asset_index_discovered
                .iter()
                .flat_map(|(asset_type, asset_records)| {
                    let asset_type = *asset_type;
                    asset_records
                        .iter()
                        .map(move |asset_record| (asset_type, asset_record))
                })
                .for_each(|(asset_type, asset_record)| {
                    let asset_id = asset_id_mappings.insert(asset_record.asset_slug.clone());

                    debug!(
                        "Asset ID ({:?}): slug: `{}`, type: `{:?}`",
                        asset_id, &asset_record.asset_slug, asset_type
                    );

                    asset_type_mappings.insert(asset_id, asset_type);
                    asset_load_stage.insert(asset_id, LoadStage::New);
                    asset_id_to_path.insert(asset_id, asset_record.path.clone());
                });

            *asset_index = Some(asset_index_discovered);
        }
    }
}

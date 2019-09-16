use std::path::PathBuf;

use amethyst::{
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::AssetDiscovery;
use asset_model::config::AssetIndex;
use derivative::Derivative;
use derive_new::new;
use log::debug;
use typename_derive::TypeName;

/// Discovers assets and writes to `Option<AssetIndex>`.
#[derive(Debug, Default, TypeName, new)]
pub struct AssetDiscoverySystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDiscoverySystemData<'s> {
    /// `Option<AssetIndex>` resource.
    #[derivative(Debug = "ignore")]
    asset_index: Write<'s, Option<AssetIndex>>,
}

impl<'s> System<'s> for AssetDiscoverySystem {
    type SystemData = AssetDiscoverySystemData<'s>;

    fn run(&mut self, AssetDiscoverySystemData { mut asset_index }: Self::SystemData) {
        // TODO: Do a diff between existing index and directory based on a file watch / notify.
        // TODO: See <https://github.com/polachok/derive-diff>
        if asset_index.is_none() {
            let asset_index_discovered = AssetDiscovery::asset_index(&self.assets_dir);
            debug!("Indexed assets: {:?}", &asset_index_discovered);
            *asset_index = Some(asset_index_discovered);
        }
    }
}

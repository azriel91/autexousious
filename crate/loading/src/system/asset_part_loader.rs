use amethyst::ecs::SystemData;
use asset_model::loaded::AssetId;
use loading_model::loaded::LoadStage;

use crate::AssetLoadingResources;

/// Loads part of an asset.
///
/// This is a partial GAT hack, combined with
///
/// For GAT status, see:
///
/// * <https://users.rust-lang.org/t/17444>
/// * <https://github.com/rust-lang/rust/issues/44265>
///
/// As of 2019-01-19, this workaround was posted:
///
/// * <https://gist.github.com/ExpHP/7a464c184c876eaf27056a83c41356ee>
pub trait AssetPartLoader<'s> {
    /// `LoadStage` that this ``AssetPartLoader` handles.
    const LOAD_STAGE: LoadStage;
    /// `SystemData` to read from the world.
    type SystemData: SystemData<'s>;

    /// Prepares collections for processing, such as setting capacities.
    fn preprocess(
        _asset_loading_resources: &mut AssetLoadingResources,
        _system_data: &mut Self::SystemData,
    ) {
    }

    /// Loads the asset part.
    fn process(
        asset_loading_resources: &mut AssetLoadingResources,
        system_data: &mut Self::SystemData,
        asset_id: AssetId,
    );

    /// Returns if the asset part is loaded.
    fn is_complete(
        asset_loading_resources: &AssetLoadingResources,
        system_data: &Self::SystemData,
        asset_id: AssetId,
    ) -> bool;
}

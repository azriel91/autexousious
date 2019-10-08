use amethyst::{
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStatus, LoadStatus};
use log::info;
use typename_derive::TypeName;

use crate::{AssetLoadingResources, SequenceComponentResources};

/// Marks loaded assets as complete.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetLoadingCompleteSystem;

/// `AssetLoadingCompleteSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingCompleteSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `SequenceComponentResources`.
    pub sequence_component_resources: SequenceComponentResources<'s>,
}

impl<'s> System<'s> for AssetLoadingCompleteSystem {
    type SystemData = AssetLoadingCompleteSystemData<'s>;

    fn run(
        &mut self,
        AssetLoadingCompleteSystemData {
            mut asset_load_status,
            asset_loading_resources,
            sequence_component_resources,
        }: Self::SystemData,
    ) {
        asset_load_status
            .iter_mut()
            .filter(|(_, load_status)| **load_status == LoadStatus::SequenceComponentLoading)
            .for_each(|(asset_id, load_status)| {
                if Self::sequence_components_loaded(&sequence_component_resources, asset_id) {
                    let asset_slug = asset_loading_resources
                        .asset_id_mappings
                        .slug(asset_id)
                        .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

                    info!("Loaded `{}`.", asset_slug);

                    *load_status = LoadStatus::Complete;
                }
            });
    }
}

impl AssetLoadingCompleteSystem {
    /// Returns whether sequence components assets have been loaded.
    fn sequence_components_loaded(
        SequenceComponentResources {
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        }: &SequenceComponentResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        macro_rules! sequence_component_loaded {
            ($handleses:ident, $assets:ident) => {{
                if let Some(handles) = $handleses.get(asset_id) {
                    handles.iter().all(|handle| $assets.get(handle).is_some())
                } else {
                    true
                }
            }};
        };

        sequence_component_loaded!(asset_wait_sequence_handles, wait_sequence_assets)
            && sequence_component_loaded!(asset_source_sequence_handles, source_sequence_assets)
            && sequence_component_loaded!(
                asset_object_acceleration_sequence_handles,
                object_acceleration_sequence_assets
            )
            && sequence_component_loaded!(
                asset_sprite_render_sequence_handles,
                sprite_render_sequence_assets
            )
            && sequence_component_loaded!(asset_body_sequence_handles, body_sequence_assets)
            && sequence_component_loaded!(
                asset_interactions_sequence_handles,
                interactions_sequence_assets
            )
            && sequence_component_loaded!(asset_spawns_sequence_handles, spawns_sequence_assets)
    }
}

use asset_model::loaded::AssetId;
use loading_model::loaded::AssetLoadStatus;
use std::marker::PhantomData;

use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStage, LoadStage, LoadStatus};
use typename_derive::TypeName;

use crate::AssetLoadingResources;

/// Loads part of a collective asset.
#[derive(Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetPartLoadingSystem<'s, R>
where
    R: SystemData<'s>,
{
    /// `LoadStage` that this System is concerned with
    load_stage: LoadStage,
    /// Function that loads the asset part.
    #[derivative(Debug = "ignore")]
    fn_process: fn(&mut AssetLoadingResources, &mut R, AssetId),
    /// Function that checks if the asset part is loaded.
    #[derivative(Debug = "ignore")]
    fn_complete: fn(&mut AssetLoadingResources, &R, AssetId),
    /// Marker.
    marker: PhantomData<&'s R>,
}

/// `AssetPartLoaderSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetPartLoaderSystemData<'s, R> {
    /// `AssetLoadStage` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Read<'s, AssetLoadStage>,
    /// `AssetLoadStatus` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// Resources needed to load the asset part.
    pub asset_part_resources: R,
}

impl<'s, R> AssetPartLoadingSystem<'s, R>
where
    R: SystemData<'s>,
{
    fn process_assets_queued(
        &self,
        AssetPartLoaderSystemData {
            asset_load_stage,
            asset_load_status,
            asset_loading_resources,
            asset_part_resources,
        }: &mut AssetPartLoaderSystemData<'_, R>,
    ) {
        asset_load_stage
            .iter()
            .filter(|(_, load_stage)| **load_stage == self.load_stage)
            .for_each(|(asset_id, _)| {
                let queued = asset_load_status
                    .get(asset_id)
                    .copied()
                    .map(|load_status| load_status == LoadStatus::Queued)
                    .unwrap_or(false);
                if queued {
                    (self.fn_process)(asset_loading_resources, asset_part_resources, asset_id);

                    asset_load_status.insert(asset_id, LoadStatus::InProgress);
                }
            });
    }

    fn process_assets_in_progress(
        &self,
        AssetPartLoaderSystemData {
            asset_load_stage,
            asset_load_status,
            asset_loading_resources,
            asset_part_resources,
        }: &mut AssetPartLoaderSystemData<'_, R>,
    ) {
        asset_load_stage
            .iter()
            .filter(|(_, load_stage)| **load_stage == self.load_stage)
            .for_each(|(asset_id, _)| {
                let in_progress = asset_load_status
                    .get(asset_id)
                    .copied()
                    .map(|load_status| load_status == LoadStatus::InProgress)
                    .unwrap_or(false);

                if in_progress {
                    (self.fn_complete)(asset_loading_resources, asset_part_resources, asset_id);

                    asset_load_status.insert(asset_id, LoadStatus::Complete);
                }
            });
    }
}

impl<'s, R> System<'s> for AssetPartLoadingSystem<'s, R>
where
    R: SystemData<'s>,
{
    type SystemData = AssetPartLoaderSystemData<'s, R>;

    fn run(&mut self, mut asset_part_loader_system_data: Self::SystemData) {
        self.process_assets_queued(&mut asset_part_loader_system_data);
        self.process_assets_in_progress(&mut asset_part_loader_system_data);
    }
}

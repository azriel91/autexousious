use std::marker::PhantomData;

use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStage, AssetLoadStatus, LoadStatus};
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::{AssetLoadingResources, AssetPartLoader};

/// Loads part of a collective asset.
#[derive(Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetPartLoadingSystem<R>
where
    R: for<'s> AssetPartLoader<'s> + TypeNameTrait,
{
    /// Marker.
    marker: PhantomData<R>,
}

/// `AssetPartLoaderSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetPartLoaderSystemData<'s, R>
where
    R: for<'sd> AssetPartLoader<'sd> + TypeNameTrait,
{
    /// `AssetLoadStage` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Read<'s, AssetLoadStage>,
    /// `AssetLoadStatus` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// Resources needed to load the asset part.
    #[derivative(Debug = "ignore")]
    pub asset_part_resources: <R as AssetPartLoader<'s>>::SystemData,
}

impl<R> AssetPartLoadingSystem<R>
where
    R: for<'s> AssetPartLoader<'s> + TypeNameTrait,
{
    fn process_assets_queued(
        &self,
        AssetPartLoaderSystemData {
            asset_load_stage,
            asset_load_status,
            asset_loading_resources,
            asset_part_resources,
        }: &mut AssetPartLoaderSystemData<R>,
    ) {
        asset_load_stage
            .iter()
            .filter(|(_, load_stage)| **load_stage == R::LOAD_STAGE)
            .for_each(|(asset_id, _)| {
                let queued = asset_load_status
                    .get(asset_id)
                    .copied()
                    .map(|load_status| load_status == LoadStatus::Queued)
                    .unwrap_or(false);
                if queued {
                    R::process(asset_loading_resources, asset_part_resources, asset_id);

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
        }: &mut AssetPartLoaderSystemData<R>,
    ) {
        asset_load_stage
            .iter()
            .filter(|(_, load_stage)| **load_stage == R::LOAD_STAGE)
            .for_each(|(asset_id, _)| {
                let in_progress = asset_load_status
                    .get(asset_id)
                    .copied()
                    .map(|load_status| load_status == LoadStatus::InProgress)
                    .unwrap_or(false);

                if in_progress
                    && R::is_complete(asset_loading_resources, asset_part_resources, asset_id)
                {
                    asset_load_status.insert(asset_id, LoadStatus::Complete);
                }
            });
    }
}

impl<'s, R> System<'s> for AssetPartLoadingSystem<R>
where
    R: for<'sd> AssetPartLoader<'sd> + TypeNameTrait,
{
    type SystemData = AssetPartLoaderSystemData<'s, R>;

    fn run(&mut self, mut asset_part_loader_system_data: Self::SystemData) {
        R::preprocess(
            &mut asset_part_loader_system_data.asset_loading_resources,
            &mut asset_part_loader_system_data.asset_part_resources,
        );
        self.process_assets_queued(&mut asset_part_loader_system_data);
        self.process_assets_in_progress(&mut asset_part_loader_system_data);
    }
}

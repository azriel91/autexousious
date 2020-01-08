use amethyst::{
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStage, AssetLoadStatus, LoadStatus};

/// Progresses a collective asset through load stages as each one is complete.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct AssetPartLoadingCoordinatorSystem;

/// `AssetPartLoadingCoordinatorSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetPartLoadingCoordinatorSystemData<'s> {
    /// `AssetLoadStage` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Write<'s, AssetLoadStage>,
    /// `AssetLoadStatus` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
}

impl<'s> System<'s> for AssetPartLoadingCoordinatorSystem {
    type SystemData = AssetPartLoadingCoordinatorSystemData<'s>;

    fn run(
        &mut self,
        AssetPartLoadingCoordinatorSystemData {
            mut asset_load_stage,
            mut asset_load_status,
        }: Self::SystemData,
    ) {
        asset_load_stage
            .iter_mut()
            .for_each(|(asset_id, load_stage)| {
                if let Some(next_load_stage) = load_stage.next() {
                    let ready_for_next_stage = asset_load_status
                        .get(asset_id)
                        .copied()
                        .map(|load_status| load_status == LoadStatus::Complete)
                        .unwrap_or(true);

                    if ready_for_next_stage {
                        *load_stage = next_load_stage;
                        asset_load_status.insert(asset_id, LoadStatus::Queued);
                    }
                }
            });
    }
}

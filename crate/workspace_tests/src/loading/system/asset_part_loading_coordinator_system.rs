#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{SystemData, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application_test_support::AssetQueries;
    use asset_model::loaded::AssetId;
    use loading_model::loaded::{LoadStage, LoadStatus};

    use loading::{AssetPartLoadingCoordinatorSystem, AssetPartLoadingCoordinatorSystemData};

    #[test]
    fn progresses_new_assets_to_next_stage() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::New,
                load_status: None,
            },
            ExpectedParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: LoadStatus::Queued,
            },
        )
    }

    #[test]
    fn progresses_complete_assets_to_next_stage() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: Some(LoadStatus::Complete),
            },
            ExpectedParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::Queued,
            },
        )
    }

    #[test]
    fn does_not_progress_queued_assets() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: Some(LoadStatus::Queued),
            },
            ExpectedParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: LoadStatus::Queued,
            },
        )
    }

    #[test]
    fn does_not_progress_in_progress_assets() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: Some(LoadStatus::InProgress),
            },
            ExpectedParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: LoadStatus::InProgress,
            },
        )
    }

    fn run_test(
        SetupParams {
            load_stage: load_stage_setup,
            load_status: load_status_setup,
        }: SetupParams,
        ExpectedParams {
            load_stage: load_stage_expected,
            load_status: load_status_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(AssetPartLoadingCoordinatorSystem::new(), "", &[])
            .with_setup(AssetPartLoadingCoordinatorSystemData::setup)
            .with_setup(move |world| {
                let asset_id = AssetQueries::id_generate_any(world);
                {
                    let asset_part_loading_coordinator_system_data =
                        world.system_data::<AssetPartLoadingCoordinatorSystemData<'_>>();

                    let AssetPartLoadingCoordinatorSystemData {
                        mut asset_load_stage,
                        mut asset_load_status,
                    } = asset_part_loading_coordinator_system_data;

                    asset_load_stage.insert(asset_id, load_stage_setup);

                    if let Some(load_status) = load_status_setup {
                        asset_load_status.insert(asset_id, load_status);
                    }
                };

                world.insert(asset_id);
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();

                let AssetPartLoadingCoordinatorSystemData {
                    asset_load_stage,
                    asset_load_status,
                } = world.system_data::<AssetPartLoadingCoordinatorSystemData<'_>>();

                assert_eq!(
                    Some(load_stage_expected),
                    asset_load_stage.get(asset_id).copied()
                );
                assert_eq!(
                    Some(load_status_expected),
                    asset_load_status.get(asset_id).copied()
                );
            })
            .run()
    }

    struct SetupParams {
        load_stage: LoadStage,
        load_status: Option<LoadStatus>,
    }

    struct ExpectedParams {
        load_stage: LoadStage,
        load_status: LoadStatus,
    }
}

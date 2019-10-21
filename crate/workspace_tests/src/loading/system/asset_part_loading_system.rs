#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, WorldExt, Write},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application_test_support::AssetQueries;
    use asset_model::loaded::AssetId;
    use loading::{AssetLoadingResources, AssetPartLoader};
    use loading_model::loaded::{AssetLoadStage, AssetLoadStatus, LoadStage, LoadStatus};
    use slotmap::SecondaryMap;
    use typename_derive::TypeName;

    use loading::AssetPartLoadingSystem;

    #[test]
    fn processes_queued_assets_for_asset_part_loader_stage() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::Queued,
                is_complete: false,
            },
            ExpectedParams {
                process_invoked: true,
                load_status: LoadStatus::InProgress,
            },
        )
    }

    #[test]
    fn ignores_in_progress_assets_for_asset_part_loader_stage() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::InProgress,
                is_complete: false,
            },
            ExpectedParams {
                process_invoked: false,
                load_status: LoadStatus::InProgress,
            },
        )
    }

    #[test]
    fn ignores_queued_assets_for_other_load_stage() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::AssetDefinitionLoading,
                load_status: LoadStatus::Queued,
                is_complete: false,
            },
            ExpectedParams {
                process_invoked: false,
                load_status: LoadStatus::Queued,
            },
        )
    }

    #[test]
    fn does_not_double_process_assets_in_progress() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::InProgress,
                is_complete: false,
            },
            ExpectedParams {
                process_invoked: false,
                load_status: LoadStatus::InProgress,
            },
        )
    }

    #[test]
    fn completes_in_progress_assets_when_is_complete() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::InProgress,
                is_complete: true,
            },
            ExpectedParams {
                process_invoked: false,
                load_status: LoadStatus::Complete,
            },
        )
    }

    #[test]
    fn completes_assets_from_queued_to_complete_in_single_run() -> Result<(), Error> {
        run_test(
            SetupParams {
                load_stage: LoadStage::IdMapping,
                load_status: LoadStatus::Queued,
                is_complete: true,
            },
            ExpectedParams {
                process_invoked: true,
                load_status: LoadStatus::Complete,
            },
        )
    }

    fn run_test(
        SetupParams {
            load_stage,
            load_status: load_status_setup,
            is_complete,
        }: SetupParams,
        ExpectedParams {
            process_invoked,
            load_status: load_status_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                AssetPartLoadingSystem::<TestAssetPartLoader>::new(),
                "",
                &[],
            )
            .with_effect(move |world| {
                let asset_id = AssetQueries::id_generate_any(world);

                {
                    let (mut asset_load_stage, mut asset_mock_load_data, mut asset_load_status) =
                        world.system_data::<(
                            Write<'_, AssetLoadStage>,
                            Write<'_, SecondaryMap<AssetId, MockLoadData>>,
                            Write<'_, AssetLoadStatus>,
                        )>();

                    let mock_load_data = MockLoadData {
                        is_complete,
                        ..Default::default()
                    };

                    asset_load_stage.insert(asset_id, load_stage);
                    asset_mock_load_data.insert(asset_id, mock_load_data);
                    asset_load_status.insert(asset_id, load_status_setup);
                }

                world.insert(asset_id);
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();

                let (asset_mock_load_data, asset_load_status) = world.system_data::<(
                    Read<'_, SecondaryMap<AssetId, MockLoadData>>,
                    Read<'_, AssetLoadStatus>,
                )>();
                let mock_load_data = asset_mock_load_data
                    .get(asset_id)
                    .copied()
                    .expect("Expected `MockLoadData` to exist.");
                let load_status_actual = asset_load_status
                    .get(asset_id)
                    .copied()
                    .expect("Expected `LoadStatus` to exist.");

                assert_eq!(process_invoked, mock_load_data.process_invoked);
                assert_eq!(load_status_expected, load_status_actual);
            })
            .run()
    }

    #[derive(TypeName)]
    struct TestAssetPartLoader;

    impl<'s> AssetPartLoader<'s> for TestAssetPartLoader {
        const LOAD_STAGE: LoadStage = LoadStage::IdMapping;
        type SystemData = Write<'s, SecondaryMap<AssetId, MockLoadData>>;

        fn process(
            _asset_loading_resources: &mut AssetLoadingResources,
            asset_mock_load_data: &mut Self::SystemData,
            asset_id: AssetId,
        ) {
            let mut mock_load_data = asset_mock_load_data
                .get_mut(asset_id)
                .expect("Expected `MockLoadData` to exist.");
            (*mock_load_data).process_invoked = true;
        }

        fn is_complete(
            _: &AssetLoadingResources,
            asset_mock_load_data: &Self::SystemData,
            asset_id: AssetId,
        ) -> bool {
            let mock_load_data = asset_mock_load_data
                .get(asset_id)
                .expect("Expected `MockLoadData` to exist.");
            (*mock_load_data).is_complete
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct MockLoadData {
        process_invoked: bool,
        is_complete: bool,
    }

    struct SetupParams {
        load_stage: LoadStage,
        load_status: LoadStatus,
        is_complete: bool,
    }

    struct ExpectedParams {
        process_invoked: bool,
        load_status: LoadStatus,
    }
}

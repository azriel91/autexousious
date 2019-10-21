#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        ecs::{Read, SystemData, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error, State, StateData, Trans,
    };
    use amethyst_test::{AmethystApplication, GameUpdate};
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use assets_test::{ASSETS_PATH, MAP_EMPTY_SLUG, MAP_FADE_SLUG};
    use game_model::play::GameEntities;
    use loading::LoadingBundle;
    use loading_model::loaded::{AssetLoadStage, LoadStage};
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;

    use game_loading::{
        GameLoadingStatus, MapSelectionSpawningSystem, MapSelectionSpawningSystemData,
    };

    #[test]
    fn returns_if_map_already_loaded() -> Result<(), Error> {
        run_test(
            SetupParams {
                setup_variant: SetupVariant::Loaded,
            },
            ExpectedParams {
                map_loaded: true,
                layer_entities_should_exist: false,
            },
        )
    }

    #[test]
    fn spawns_map_layers_when_they_havent_been_spawned() -> Result<(), Error> {
        run_test(
            SetupParams {
                setup_variant: SetupVariant::NotLoaded(&*MAP_FADE_SLUG),
            },
            ExpectedParams {
                map_loaded: true,
                layer_entities_should_exist: true,
            },
        )
    }

    #[test]
    fn spawns_map_that_has_no_layers() -> Result<(), Error> {
        run_test(
            SetupParams {
                setup_variant: SetupVariant::NotLoaded(&*MAP_EMPTY_SLUG),
            },
            ExpectedParams {
                map_loaded: true,
                layer_entities_should_exist: false,
            },
        )
    }

    fn run_test(
        SetupParams { setup_variant }: SetupParams<'static>,
        ExpectedParams {
            map_loaded: map_loaded_expected,
            layer_entities_should_exist,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        let (slug, map_loaded_setup) = match setup_variant {
            SetupVariant::NotLoaded(slug) => (slug, false),
            SetupVariant::Loaded => (&*MAP_EMPTY_SLUG, true),
        };

        let wait_for_load = WaitForLoad { slug: slug.clone() };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_state(|| wait_for_load)
            .with_effect(setup_system_data)
            .with_effect(move |world| setup_map_selection(world, slug))
            .with_effect(move |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.map_loaded = map_loaded_setup;
                world.insert(game_loading_status);
            })
            .with_system_single(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                if map_loaded_expected {
                    assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
                } else {
                    assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
                }

                if layer_entities_should_exist {
                    assert!(!world.read_resource::<GameEntities>().map_layers.is_empty());
                } else {
                    assert!(world.read_resource::<GameEntities>().map_layers.is_empty());
                }
            })
            .run()
    }

    fn setup_system_data(world: &mut World) {
        MapSelectionSpawningSystemData::setup(world);
    }

    fn setup_map_selection(world: &mut World, slug: &AssetSlug) {
        let map_asset_id = world
            .read_resource::<AssetIdMappings>()
            .id(slug)
            .copied()
            .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug));

        world.insert(MapSelection::Id(map_asset_id));
        world.insert(MapSelectionStatus::Confirmed);
    }

    struct SetupParams<'s> {
        setup_variant: SetupVariant<'s>,
    }

    struct ExpectedParams {
        map_loaded: bool,
        layer_entities_should_exist: bool,
    }

    enum SetupVariant<'s> {
        NotLoaded(&'s AssetSlug),
        Loaded,
    }

    #[derive(Debug)]
    struct WaitForLoad {
        slug: AssetSlug,
    }
    impl<T, E> State<T, E> for WaitForLoad
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let (asset_id_mappings, asset_load_stage) = data
                .world
                .system_data::<(Read<'_, AssetIdMappings>, Read<'_, AssetLoadStage>)>();
            if let Some(LoadStage::Complete) = asset_id_mappings
                .id(&self.slug)
                .and_then(|asset_id| asset_load_stage.get(*asset_id))
            {
                Trans::Pop
            } else {
                Trans::None
            }
        }
    }
}

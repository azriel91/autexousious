use amethyst::{
    ecs::{ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use map_selection_model::MapSelection;
use typename_derive::TypeName;

use crate::{
    GameLoadingStatus, MapLayerComponentStorages, MapLayerEntitySpawner, MapSpawningResources,
};

/// Spawns map entities based on the map selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSpawningSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionSpawningSystemData<'s> {
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `MapSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub map_spawning_resources: MapSpawningResources<'s>,
    /// `MapLayerComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub map_layer_component_storages: MapLayerComponentStorages<'s>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
}

impl<'s> System<'s> for MapSelectionSpawningSystem {
    type SystemData = MapSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionSpawningSystemData {
            mut game_loading_status,
            map_selection,
            map_spawning_resources,
            mut map_layer_component_storages,
            mut game_entities,
        }: Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        // TODO: implement Random
        let map_layer_entities = MapLayerEntitySpawner::spawn_system(
            &map_spawning_resources,
            &mut map_layer_component_storages,
            map_selection
                .asset_id()
                .expect("Expected `MapSelection` to contain ID."),
        );

        game_entities.map_layers = map_layer_entities;
        game_loading_status.map_loaded = true;
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        ecs::{SystemData, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use assets_test::{ASSETS_PATH, MAP_EMPTY_SLUG, MAP_FADE_SLUG};
    use game_model::play::GameEntities;
    use loading::LoadingBundle;
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;

    use super::{MapSelectionSpawningSystem, MapSelectionSpawningSystemData};
    use crate::GameLoadingStatus;

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
            SetupVariant::NotLoaded(slug) => (Some(slug), false),
            SetupVariant::Loaded => (None, true),
        };

        let mut amethyst_application = AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(setup_system_data);

        if let Some(slug) = slug {
            amethyst_application =
                amethyst_application.with_effect(move |world| setup_map_selection(world, slug))
        }

        amethyst_application
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
            .run_isolated()
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
}

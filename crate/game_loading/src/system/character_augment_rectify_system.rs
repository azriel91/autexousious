use amethyst::{
    assets::PrefabData,
    ecs::{
        Entities, Entity, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, World, Write,
        WriteStorage,
    },
    shred::{ResourceId, SystemData},
    utils::removal::Removal,
};
use camera_model::play::CameraTracked;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_play_hud::{CpBarPrefab, HpBarPrefab};
use game_play_model::{GamePlayEntity, GamePlayEntityId};
use kinematic_model::config::Position;
use map_model::loaded::AssetMapBounds;
use map_selection_model::MapSelection;
use typename_derive::TypeName;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterAugmentRectifySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterAugmentRectifySystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `CameraTracked` components.
    #[derivative(Debug = "ignore")]
    pub camera_trackeds: WriteStorage<'s, CameraTracked>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `LazyUpdate` resource.
    ///
    /// This is used because both the `HpBarPrefab` and `CpBarPrefab` request `Write` access to the
    /// same resources.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
}

impl CharacterAugmentRectifySystem {
    fn hp_bar_augment(world: &World, game_object_entity: Entity) {
        let (entities, mut hp_bar_prefab_system_data, mut game_play_entities) = world
            .system_data::<(
                Entities<'_>,
                <HpBarPrefab as PrefabData<'_>>::SystemData,
                WriteStorage<'_, GamePlayEntity>,
            )>();

        let hp_bar_entity = entities.create();
        let hp_bar_prefab = HpBarPrefab::new(game_object_entity);
        hp_bar_prefab
            .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[], &[])
            .expect("`HpBarPrefab` failed to augment entity.");
        game_play_entities
            .insert(hp_bar_entity, Removal::new(GamePlayEntityId))
            .expect("Failed to insert `GamePlayEntity` component.");
    }

    fn cp_bar_augment(world: &World, game_object_entity: Entity) {
        let (entities, mut cp_bar_prefab_system_data, mut game_play_entities) = world
            .system_data::<(
                Entities<'_>,
                <CpBarPrefab as PrefabData<'_>>::SystemData,
                WriteStorage<'_, GamePlayEntity>,
            )>();

        let cp_bar_entity = entities.create();
        let cp_bar_prefab = CpBarPrefab::new(game_object_entity);
        cp_bar_prefab
            .add_to_entity(cp_bar_entity, &mut cp_bar_prefab_system_data, &[], &[])
            .expect("`CpBarPrefab` failed to augment entity.");
        game_play_entities
            .insert(cp_bar_entity, Removal::new(GamePlayEntityId))
            .expect("Failed to insert `GamePlayEntity` component.");
    }
}

impl<'s> System<'s> for CharacterAugmentRectifySystem {
    type SystemData = CharacterAugmentRectifySystemData<'s>;

    fn run(
        &mut self,
        CharacterAugmentRectifySystemData {
            entities,
            mut game_loading_status,
            map_selection,
            asset_map_bounds,
            input_controlleds,
            mut camera_trackeds,
            mut positions,
            lazy_update,
        }: Self::SystemData,
    ) {
        // TODO: Entities may not have health_points component -- see the second join()

        // TODO: We may actually want this system to run during gameplay, e.g. when changing which
        // game object is controlled.

        if game_loading_status.character_augment_status != CharacterAugmentStatus::Rectify {
            return;
        }

        // Read map to determine bounds where the characters can be spawned.
        let (width, height, depth) = {
            asset_map_bounds
                .get(
                    map_selection
                        .asset_id()
                        .expect("Expected map selection to have an `AssetId`."),
                )
                .map(|bounds| {
                    (
                        bounds.width as f32,
                        bounds.height as f32,
                        bounds.depth as f32,
                    )
                })
                .expect("Expected map selection to have `MapBounds`.")
        };

        // This `Position` moves the entity to the middle of a screen wide map.
        let position = Position::<f32>::new(width / 2., height / 2., depth / 2.);

        (&entities, &input_controlleds)
            .join()
            .for_each(|(entity, _)| {
                // Set character `position` based on the map.
                positions
                    .insert(entity, position)
                    .expect("Failed to insert `Position<f32>` component.");

                // Track player with camera.
                camera_trackeds
                    .insert(entity, CameraTracked)
                    .expect("Failed to insert `CameraTracked` component.");

                lazy_update.exec(move |world| Self::hp_bar_augment(world, entity));
                lazy_update.exec(move |world| Self::cp_bar_augment(world, entity));
            });

        game_loading_status.character_augment_status = CharacterAugmentStatus::Complete;
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        <HpBarPrefab as PrefabData<'_>>::SystemData::setup(world);
        <CpBarPrefab as PrefabData<'_>>::SystemData::setup(world);
        <WriteStorage<'_, GamePlayEntity>>::setup(world);
    }
}

#[cfg(test)]
mod tests {
    use game_model::play::GameEntities;
    use std::collections::HashMap;

    use amethyst::{
        assets::Processor,
        audio::Source,
        core::TransformBundle,
        ecs::{Join, Read, ReadStorage, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shred::SystemData,
        window::ScreenDimensions,
        Error, State, StateData, Trans,
    };
    use amethyst_test::{
        AmethystApplication, GameUpdate, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH,
    };
    use application_event::{AppEvent, AppEventReader};
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::{ASSETS_PATH, MAP_FADE_SLUG};
    use audio_loading::AudioLoadingBundle;
    use character_loading::CharacterLoadingBundle;
    use character_selection_model::CharacterSelections;
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use energy_loading::EnergyLoadingBundle;
    use game_input_model::ControlBindings;
    use game_play_hud::{CpBar, HpBar};
    use kinematic_loading::KinematicLoadingBundle;
    use kinematic_model::config::Position;
    use loading::{LoadingBundle, LoadingState};
    use loading_model::loaded::{AssetLoadStatus, LoadStatus};
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use object_type::ObjectType;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use super::{CharacterAugmentRectifySystem, CharacterAugmentRectifySystemData};
    use crate::{CharacterAugmentStatus, CharacterSelectionSpawningSystem, GameLoadingStatus};

    #[test]
    fn returns_if_augment_status_is_not_rectify() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.insert(game_loading_status);
            },
            |world| {
                let char_entity = world
                    .read_resource::<GameEntities>()
                    .objects
                    .get(&ObjectType::Character)
                    .expect("Expected `Character` entities to exist.")
                    .iter()
                    .next()
                    .copied()
                    .expect("Expected character entity to exist.");
                assert_eq!(
                    // Default is inserted by character augmenter.
                    Some(Position::<f32>::new(0., 0., 0.)).as_ref(),
                    world.read_storage::<Position<f32>>().get(char_entity)
                );
            },
        )
    }

    #[test]
    fn updates_position_to_middle_of_map() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.insert(game_loading_status);
            },
            |world| {
                let char_entity = world
                    .read_resource::<GameEntities>()
                    .objects
                    .get(&ObjectType::Character)
                    .expect("Expected `Character` entities to exist.")
                    .iter()
                    .next()
                    .copied()
                    .expect("Expected character entity to exist.");
                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    // See assets_test/assets/test/map/fade/map.yaml
                    Position::<f32>::new(400., 200., 100.),
                    *world
                        .read_storage::<Position<f32>>()
                        .get(char_entity)
                        .expect("Expected entity to have position.")
                );
                assert_eq!(
                    CharacterAugmentStatus::Complete,
                    world
                        .read_resource::<GameLoadingStatus>()
                        .character_augment_status
                );
            },
        )
    }

    #[test]
    fn creates_hp_and_cp_bar_entities_per_character_selection() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.insert(game_loading_status);
            },
            |world| {
                let (hp_bars, cp_bars) =
                    world.system_data::<(ReadStorage<'_, HpBar>, ReadStorage<'_, CpBar>)>();
                assert_eq!(1, (&hp_bars).join().count());
                assert_eq!(1, (&cp_bars).join().count());
            },
        )
    }

    fn run_test<FnS, FnA>(fn_setup: FnS, fn_assert: FnA) -> Result<(), Error>
    where
        FnS: Fn(&mut World) + Send + Sync + 'static,
        FnA: Fn(&mut World) + Send + Sync + 'static,
    {
        let wait_for_load = WaitForLoad {
            slug: MAP_FADE_SLUG.clone(),
        };

        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(KinematicLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| CharacterAugmentRectifySystemData::setup(world))
            .with_state(|| LoadingState::new(PopState))
            .with_state(|| wait_for_load)
            .with_effect(|world| setup_map_selection(world, &*MAP_FADE_SLUG))
            .with_effect(|world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.insert(game_loading_status);

                let asset_id = {
                    let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
                    asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Character))
                        .next()
                        .copied()
                        .expect("Expected at least one character to be loaded.")
                };
                let mut character_selections = HashMap::new();
                character_selections.insert(123, asset_id);
                let character_selections = CharacterSelections::new(character_selections);
                world.insert(character_selections);
            })
            .with_system_single(CharacterSelectionSpawningSystem::new(), "", &[])
            .with_effect(fn_setup)
            .with_system_single(
                CharacterAugmentRectifySystem,
                CharacterAugmentRectifySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(fn_assert)
            .run_isolated()
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

            let (asset_id_mappings, asset_load_status) = data
                .world
                .system_data::<(Read<'_, AssetIdMappings>, Read<'_, AssetLoadStatus>)>();
            if let Some(LoadStatus::Complete) = asset_id_mappings
                .id(&self.slug)
                .and_then(|asset_id| asset_load_status.get(*asset_id))
            {
                Trans::Pop
            } else {
                Trans::None
            }
        }
    }
}

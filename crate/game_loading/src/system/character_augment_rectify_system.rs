use amethyst::{
    assets::{AssetStorage, PrefabData},
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    utils::removal::Removal,
};
use character_prefab::CharacterPrefabHandle;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_play_hud::{CpBarPrefab, HpBarPrefab};
use game_play_model::{GamePlayEntity, GamePlayEntityId};
use kinematic_model::config::Position;
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use shred_derive::SystemData;
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
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub map_assets: Read<'s, AssetStorage<Map>>,
    /// `CharacterPrefabHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_prefab_handles: ReadStorage<'s, CharacterPrefabHandle>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `GamePlayEntity` components.
    #[derivative(Debug = "ignore")]
    pub game_play_entities: WriteStorage<'s, GamePlayEntity>,
    /// `HpBarPrefabSystemData`.
    #[derivative(Debug = "ignore")]
    pub hp_bar_prefab_system_data: <HpBarPrefab as PrefabData<'s>>::SystemData,
    /// `CpBarPrefabSystemData`.
    #[derivative(Debug = "ignore")]
    pub cp_bar_prefab_system_data: <CpBarPrefab as PrefabData<'s>>::SystemData,
}

impl<'s> System<'s> for CharacterAugmentRectifySystem {
    type SystemData = CharacterAugmentRectifySystemData<'s>;

    fn run(
        &mut self,
        CharacterAugmentRectifySystemData {
            entities,
            mut game_loading_status,
            map_selection,
            map_assets,
            character_prefab_handles,
            input_controlleds,
            mut positions,
            mut game_play_entities,
            mut hp_bar_prefab_system_data,
            mut cp_bar_prefab_system_data,
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
            map_assets
                .get(map_selection.handle())
                .map(|map| {
                    let bounds = &map.definition.header.bounds;
                    (
                        bounds.width as f32,
                        bounds.height as f32,
                        bounds.depth as f32,
                    )
                })
                .expect("Expected map to be loaded.")
        };

        // This `Position` moves the entity to the middle of a screen wide map.
        let position = Position::<f32>::new(width / 2., height / 2., depth / 2.);

        (&entities, &character_prefab_handles)
            .join()
            .for_each(|(entity, _)| {
                // Set character `position` based on the map.
                positions
                    .insert(entity, position)
                    .expect("Failed to insert position for character.");
            });

        (&entities, &input_controlleds, &character_prefab_handles)
            .join()
            .for_each(|(game_object_entity, _, _)| {
                let hp_bar_entity = entities.create();
                let hp_bar_prefab = HpBarPrefab::new(game_object_entity);
                hp_bar_prefab
                    .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[], &[])
                    .expect("`HpBarPrefab` failed to augment entity.");
                game_play_entities
                    .insert(hp_bar_entity, Removal::new(GamePlayEntityId))
                    .expect("Failed to insert `GamePlayEntity` component.");

                let cp_bar_entity = entities.create();
                let cp_bar_prefab = CpBarPrefab::new(game_object_entity);
                cp_bar_prefab
                    .add_to_entity(cp_bar_entity, &mut cp_bar_prefab_system_data, &[], &[])
                    .expect("`CpBarPrefab` failed to augment entity.");
                game_play_entities
                    .insert(cp_bar_entity, Removal::new(GamePlayEntityId))
                    .expect("Failed to insert `GamePlayEntity` component.");
            });

        game_loading_status.character_augment_status = CharacterAugmentStatus::Complete;
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{Prefab, Processor},
        audio::Source,
        core::TransformBundle,
        ecs::{Builder, Entity, Join, ReadStorage, SystemData, World},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use application_event::{AppEvent, AppEventReader};
    use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
    use assets_test::{ASSETS_PATH, CHAR_BAT_SLUG, MAP_FADE_SLUG};
    use character_loading::{CharacterLoadingBundle, CHARACTER_PROCESSOR};
    use character_prefab::{CharacterPrefab, CharacterPrefabBundle};
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use game_input::InputControlled;
    use game_input_model::ControlBindings;
    use game_model::loaded::MapPrefabs;
    use game_play_hud::{CpBar, HpBar};
    use kinematic_model::config::Position;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use super::{CharacterAugmentRectifySystem, CharacterAugmentRectifySystemData};
    use crate::{CharacterAugmentStatus, GameLoadingStatus};

    #[test]
    fn returns_if_augment_status_is_not_rectify() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.add_resource(game_loading_status);

                let snh = SlugAndHandle::<Prefab<CharacterPrefab>>::from((
                    &*world,
                    CHAR_BAT_SLUG.clone(),
                ));
                let char_entity = world.create_entity().with(snh.handle).build();

                world.add_resource(char_entity);
            },
            |world| {
                let char_entity = *world.read_resource::<Entity>();
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
                world.add_resource(game_loading_status);

                let snh = SlugAndHandle::<Prefab<CharacterPrefab>>::from((
                    &*world,
                    CHAR_BAT_SLUG.clone(),
                ));
                let char_entity = world.create_entity().with(snh.handle).build();

                world.add_resource(char_entity);
            },
            |world| {
                let char_entity = *world.read_resource::<Entity>();
                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    // See assets_test/assets/test/map/fade/map.toml
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
                world.add_resource(game_loading_status);

                let snh = SlugAndHandle::<Prefab<CharacterPrefab>>::from((
                    &*world,
                    CHAR_BAT_SLUG.clone(),
                ));
                let char_entity = world
                    .create_entity()
                    .with(snh.handle)
                    .with(InputControlled::new(0))
                    .build();

                world.add_resource(char_entity);
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
        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(
                CharacterPrefabBundle::new()
                    .with_system_dependencies(&[String::from(CHARACTER_PROCESSOR)]),
            )
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_setup(|world| CharacterAugmentRectifySystemData::setup(&mut world.res))
            .with_state(|| LoadingState::new(PopState))
            .with_setup(map_selection(MAP_FADE_SLUG.clone()))
            .with_setup(fn_setup)
            .with_system_single(
                CharacterAugmentRectifySystem,
                CharacterAugmentRectifySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(fn_assert)
            .run_isolated()
    }

    /// Returns a function that adds a `MapSelection` and `MapSelectionStatus::Confirmed`.
    ///
    /// See `application_test_support::SetupFunction`.
    ///
    /// # Parameters
    ///
    /// * `slug`: Asset slug of the map to select.
    fn map_selection(slug: AssetSlug) -> impl Fn(&mut World) {
        move |world| {
            let slug_and_handle = {
                let map_handle = world
                    .read_resource::<MapPrefabs>()
                    .get(&slug)
                    .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug))
                    .clone();

                SlugAndHandle::from((slug.clone(), map_handle))
            };

            world.add_resource(MapSelection::Id(slug_and_handle));
            world.add_resource(MapSelectionStatus::Confirmed);
        }
    }
}

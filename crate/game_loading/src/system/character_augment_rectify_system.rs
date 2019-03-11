use amethyst::{
    assets::{AssetStorage, PrefabData},
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    utils::removal::Removal,
};
use character_loading::CharacterPrefabHandle;
use derive_new::new;
use game_input::InputControlled;
use game_play_hud::HpBarPrefab;
use game_play_model::{GamePlayEntity, GamePlayEntityId};
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use object_model::entity::{HealthPoints, Position};
use typename_derive::TypeName;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterAugmentRectifySystem;

type CharacterAugmentRectifySystemData<'s> = (
    Entities<'s>,
    Write<'s, GameLoadingStatus>,
    ReadExpect<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    ReadStorage<'s, CharacterPrefabHandle>,
    ReadStorage<'s, InputControlled>,
    ReadStorage<'s, HealthPoints>,
    WriteStorage<'s, Position<f32>>,
    WriteStorage<'s, GamePlayEntity>,
    <HpBarPrefab as PrefabData<'s>>::SystemData,
);

impl<'s> System<'s> for CharacterAugmentRectifySystem {
    type SystemData = CharacterAugmentRectifySystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            mut game_loading_status,
            map_selection,
            loaded_maps,
            character_prefab_handles,
            input_controlleds,
            health_pointses,
            mut positions,
            mut game_play_entities,
            mut hp_bar_prefab_system_data,
        ): Self::SystemData,
    ) {
        if game_loading_status.character_augment_status != CharacterAugmentStatus::Rectify {
            return;
        }

        // Read map to determine bounds where the characters can be spawned.
        let (width, height, depth) = {
            loaded_maps
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

        (&entities, &input_controlleds, &health_pointses)
            .join()
            .for_each(|(game_object_entity, _, _)| {
                let hp_bar_entity = entities.create();

                let hp_bar_prefab = HpBarPrefab::new(game_object_entity);
                hp_bar_prefab
                    .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[])
                    .expect("`HpBarPrefab` failed to augment entity.");

                game_play_entities
                    .insert(hp_bar_entity, Removal::new(GamePlayEntityId))
                    .expect("Failed to insert `GamePlayEntity` component.");
            });

        game_loading_status.character_augment_status = CharacterAugmentStatus::Complete;
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::Prefab,
        ecs::{Builder, Entity, World},
        Error,
    };
    use amethyst_test::{AmethystApplication, PopState};
    use application_event::{AppEvent, AppEventReader};
    use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_MAP_FADE_SLUG, ASSETS_PATH};
    use character_loading::{CharacterLoadingBundle, CharacterPrefab};
    use collision_loading::CollisionLoadingBundle;
    use game_model::loaded::MapAssets;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use object_model::entity::Position;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;

    use super::CharacterAugmentRectifySystem;
    use crate::{CharacterAugmentStatus, GameLoadingStatus};

    #[test]
    fn returns_if_augment_status_is_not_rectify() -> Result<(), Error> {
        AmethystApplication::render_base("returns_if_augment_status_is_not_rectify", false)
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_state(|| LoadingState::new(PopState))
            .with_setup(map_selection(ASSETS_MAP_FADE_SLUG.clone()))
            .with_setup(|world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.add_resource(game_loading_status);

                let snh = SlugAndHandle::<Prefab<CharacterPrefab>>::from((
                    &*world,
                    ASSETS_CHAR_BAT_SLUG.clone(),
                ));
                let char_entity = world.create_entity().with(snh.handle).build();

                world.add_resource(char_entity);
            })
            .with_system_single(
                CharacterAugmentRectifySystem,
                CharacterAugmentRectifySystem::type_name(),
                &[],
            )
            .with_assertion(|world| {
                let char_entity = *world.read_resource::<Entity>();
                assert_eq!(
                    // Default is inserted by character augmenter.
                    Some(Position::<f32>::new(0., 0., 0.)).as_ref(),
                    world.read_storage::<Position<f32>>().get(char_entity)
                );
            })
            .run()
    }

    #[test]
    fn updates_position_to_middle_of_map() -> Result<(), Error> {
        AmethystApplication::render_base("updates_position_to_middle_of_map", false)
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_state(|| LoadingState::new(PopState))
            .with_setup(map_selection(ASSETS_MAP_FADE_SLUG.clone()))
            .with_setup(|world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.add_resource(game_loading_status);

                let snh = SlugAndHandle::<Prefab<CharacterPrefab>>::from((
                    &*world,
                    ASSETS_CHAR_BAT_SLUG.clone(),
                ));
                let char_entity = world.create_entity().with(snh.handle).build();

                world.add_resource(char_entity);
            })
            .with_system_single(
                CharacterAugmentRectifySystem,
                CharacterAugmentRectifySystem::type_name(),
                &[],
            )
            .with_assertion(|world| {
                let char_entity = *world.read_resource::<Entity>();
                assert_eq!(
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
            })
            .run()
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
                    .read_resource::<MapAssets>()
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

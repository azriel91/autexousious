use amethyst::{assets::AssetStorage, ecs::prelude::*};
use character_selection_model::CharacterSelections;
use derive_new::new;
use game_input::InputControlled;
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Position, Velocity},
    loaded::Character,
    ObjectType,
};

use crate::{
    CharacterComponentStorages, CharacterEntitySpawner, GameLoadingStatus, ObjectAnimationStorages,
    ObjectComponentStorages, ObjectSpawningResources,
};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSpawningSystem;

type CharacterSelectionSpawningSystemData<'s> = (
    Write<'s, GameLoadingStatus>,
    ReadExpect<'s, MapSelection>,
    Read<'s, CharacterSelections>,
    Read<'s, AssetStorage<Map>>,
    ObjectSpawningResources<'s, Character, CharacterSequenceId>,
    CharacterComponentStorages<'s>,
    ObjectComponentStorages<'s>,
    ObjectAnimationStorages<'s, CharacterSequenceId>,
    Write<'s, GameEntities>,
);

impl<'s> System<'s> for CharacterSelectionSpawningSystem {
    type SystemData = CharacterSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        (
            mut game_loading_status,
            map_selection,
            character_selections,
            loaded_maps,
            object_spawning_resources,
            mut character_component_storages,
            mut object_component_storages,
            mut object_animation_storages,
            mut game_entities,
        ): Self::SystemData,
    ) {
        if game_loading_status.characters_loaded {
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

        // This `Position` moves the entity to the middle of a "screen wide" map.
        let position = Position::new(width / 2., height / 2., depth / 2.);
        let velocity = Velocity::default();

        let character_entities = character_selections
            .selections
            .iter()
            .map(|(controller_id, slug_and_handle)| {
                (InputControlled::new(*controller_id), slug_and_handle)
            })
            .map(|(input_controlled, slug_and_handle)| {
                CharacterEntitySpawner::spawn_system(
                    &object_spawning_resources,
                    &mut character_component_storages,
                    &mut object_component_storages,
                    &mut object_animation_storages,
                    position,
                    velocity,
                    &slug_and_handle,
                    input_controlled,
                )
            })
            .collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);

        game_loading_status.characters_loaded = true;
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use amethyst::{animation::AnimationBundle, ecs::prelude::*};
    use amethyst_test::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_MAP_FADE_SLUG, ASSETS_PATH};
    use character_selection_model::CharacterSelections;
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::{
        config::AssetSlug,
        loaded::{MapAssets, SlugAndHandle},
        play::GameEntities,
    };
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use object_loading::ObjectLoadingBundle;
    use object_model::{config::object::CharacterSequenceId, ObjectType};
    use typename::TypeName;

    use super::CharacterSelectionSpawningSystem;
    use crate::GameLoadingStatus;

    #[test]
    fn returns_if_characters_already_loaded() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("returns_if_characters_already_loaded", false)
                .with_custom_event_type::<AppEvent, AppEventReader>()
                .with_bundle(
                    AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                        "character_body_frame_acs",
                        "character_body_frame_sis",
                    )
                )
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    InteractionFrameActiveHandle,
                >::new(
                    "character_interaction_acs", "character_interaction_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_bundle(ObjectLoadingBundle::new())
                .with_state(|| LoadingState::new(ASSETS_PATH.clone(), PopState))
                .with_setup(map_selection(ASSETS_MAP_FADE_SLUG.clone()))
                .with_setup(|world| {
                    let mut game_loading_status = GameLoadingStatus::new();
                    game_loading_status.characters_loaded = true;
                    world.add_resource(game_loading_status);

                    let char_entity = world.create_entity().build();
                    let mut objects = HashMap::new();
                    objects.insert(ObjectType::Character, vec![char_entity.clone()]);

                    world.add_resource(GameEntities::new(objects, Vec::new()));
                    world.add_resource(EffectReturn(char_entity));
                })
                .with_system_single(
                    CharacterSelectionSpawningSystem,
                    CharacterSelectionSpawningSystem::type_name(),
                    &[],
                )
                .with_assertion(|world| {
                    let char_entity = &world.read_resource::<EffectReturn<Entity>>().0;
                    assert_eq!(
                        char_entity,
                        world
                            .read_resource::<GameEntities>()
                            .objects
                            .get(&ObjectType::Character)
                            .expect("Expected `ObjectType::Character` key in `GameEntities`.")
                            .iter()
                            .next()
                            .expect("Expected characters to have an entity.")
                    );
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn spawns_characters_when_they_havent_been_spawned() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawns_characters_when_they_havent_been_spawned",
                false
            )
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(
                AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                    "character_body_frame_acs",
                    "character_body_frame_sis",
                )
            )
            .with_bundle(AnimationBundle::<
                CharacterSequenceId,
                InteractionFrameActiveHandle,
            >::new(
                "character_interaction_acs", "character_interaction_sis",
            ))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_state(|| LoadingState::new(ASSETS_PATH.clone(), PopState))
            .with_setup(map_selection(ASSETS_MAP_FADE_SLUG.clone()))
            .with_setup(|world| {
                let mut character_selections = CharacterSelections::default();
                character_selections.selections.insert(
                    0,
                    SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone())),
                );
                world.add_resource(character_selections);
            })
            .with_system_single(
                CharacterSelectionSpawningSystem,
                CharacterSelectionSpawningSystem::type_name(),
                &[],
            )
            .with_assertion(|world| {
                assert!(!world
                    .read_resource::<GameEntities>()
                    .objects
                    .get(&ObjectType::Character)
                    .expect("Expected `ObjectType::Character` key in `GameEntities`.")
                    .is_empty());
                assert!(world.read_resource::<GameLoadingStatus>().characters_loaded);
            })
            .run()
            .is_ok()
        );
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

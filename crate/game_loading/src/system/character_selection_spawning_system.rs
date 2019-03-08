use amethyst::ecs::{Entities, Entity, Read, System, Write, WriteStorage};
use character_loading::CharacterPrefabHandle;
use character_selection_model::CharacterSelections;
use derive_new::new;
use game_input::InputControlled;
use game_model::play::GameEntities;
use object_model::ObjectType;
use typename_derive::TypeName;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSpawningSystem;

type CharacterSelectionSpawningSystemData<'s> = (
    Entities<'s>,
    Write<'s, GameLoadingStatus>,
    Read<'s, CharacterSelections>,
    WriteStorage<'s, CharacterPrefabHandle>,
    WriteStorage<'s, InputControlled>,
    Write<'s, GameEntities>,
);

impl<'s> System<'s> for CharacterSelectionSpawningSystem {
    type SystemData = CharacterSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            mut game_loading_status,
            character_selections,
            mut character_prefab_handles,
            mut input_controlleds,
            mut game_entities,
        ): Self::SystemData,
    ) {
        if game_loading_status.character_augment_status != CharacterAugmentStatus::Prefab {
            return;
        }

        let character_entities = character_selections
            .selections
            .iter()
            .map(|(controller_id, slug_and_handle)| {
                let entity = entities.create();

                input_controlleds
                    .insert(entity, InputControlled::new(*controller_id))
                    .expect("Failed to insert input_controlled for character.");

                character_prefab_handles
                    .insert(entity, slug_and_handle.handle.clone())
                    .expect("Failed to insert character_prefab_handle for character.");

                entity
            })
            .collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);

        game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use amethyst::ecs::prelude::*;
    use amethyst_test::{AmethystApplication, EffectReturn, PopState};
    use application_event::{AppEvent, AppEventReader};
    use asset_model::loaded::SlugAndHandle;
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use character_loading::CharacterLoadingBundle;
    use character_selection_model::CharacterSelections;
    use collision_loading::CollisionLoadingBundle;
    use game_model::play::GameEntities;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use object_model::ObjectType;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;

    use super::CharacterSelectionSpawningSystem;
    use crate::{CharacterAugmentStatus, GameLoadingStatus};

    #[test]
    fn returns_if_augment_status_is_not_prefab() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("returns_if_augment_status_is_not_prefab", false)
                .with_custom_event_type::<AppEvent, AppEventReader>()
                .with_bundle(SpriteLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_bundle(CharacterLoadingBundle::new())
                .with_state(|| LoadingState::new(PopState))
                .with_setup(|world| {
                    let mut game_loading_status = GameLoadingStatus::new();
                    game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
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
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_state(|| LoadingState::new(PopState))
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
                assert_eq!(
                    CharacterAugmentStatus::Rectify,
                    world
                        .read_resource::<GameLoadingStatus>()
                        .character_augment_status
                );
            })
            .run()
            .is_ok()
        );
    }
}

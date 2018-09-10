use amethyst::{assets::AssetStorage, ecs::prelude::*};
use character_selection::CharacterSelections;
use game_input::InputControlled;
use game_model::{loaded::CharacterAssets, play::GameEntities};
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Kinematics, Position, Velocity},
    loaded::Character,
    ObjectType,
};

use CharacterComponentStorages;
use CharacterEntitySpawner;
use GameLoadingStatus;
use ObjectComponentStorages;

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSpawningSystem;

type CharacterSelectionSpawningSystemData<'s> = (
    Write<'s, GameLoadingStatus>,
    Read<'s, CharacterSelections>,
    Read<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    Entities<'s>,
    Read<'s, CharacterAssets>,
    Read<'s, AssetStorage<Character>>,
    CharacterComponentStorages<'s>,
    ObjectComponentStorages<'s, CharacterSequenceId>,
    Write<'s, GameEntities>,
);

impl<'s> System<'s> for CharacterSelectionSpawningSystem {
    type SystemData = CharacterSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        (
            mut game_loading_status,
            character_selections,
            map_selection,
            loaded_maps,
            entities,
            character_assets,
            loaded_characters,
            mut character_component_storages,
            mut object_component_storages,
            mut game_entities,
        ): Self::SystemData,
    ) {
        if game_loading_status.characters_loaded {
            return;
        }

        // Read map to determine bounds where the characters can be spawned.
        let map_handle = map_selection
            .map_handle
            .as_ref()
            .expect("Expected map to be selected.");

        let (width, height, depth) = {
            loaded_maps
                .get(map_handle)
                .map(|map| {
                    let bounds = &map.definition.header.bounds;
                    (
                        bounds.width as f32,
                        bounds.height as f32,
                        bounds.depth as f32,
                    )
                }).expect("Expected map to be loaded.")
        };

        // This `Position` moves the entity to the middle of a "screen wide" map.
        let position = Position::new(width / 2., height / 2., depth / 2.);
        let kinematics = Kinematics::new(position, Velocity::default());

        let object_spawning_resources = (&*entities, &*character_assets, &*loaded_characters);

        let character_entities = character_selections
            .selections
            .iter()
            .map(|(controller_id, character_slug)| {
                (InputControlled::new(*controller_id), character_slug)
            }).map(|(input_controlled, character_slug)| {
                CharacterEntitySpawner::spawn_system(
                    &object_spawning_resources,
                    &mut character_component_storages,
                    &mut object_component_storages,
                    kinematics,
                    &character_slug,
                    input_controlled,
                )
            }).collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);

        game_loading_status.characters_loaded = true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::path::Path;

    use amethyst::ecs::prelude::*;
    use amethyst_test_support::{prelude::*, EmptyState};
    use application::resource::dir::ASSETS;
    use asset_loading::ASSETS_TEST_DIR;
    use character_selection::CharacterSelections;
    use game_model::{
        config::{AssetSlug, AssetSlugBuilder},
        loaded::MapAssets,
        play::GameEntities,
    };
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_selection::{MapSelection, MapSelectionStatus};
    use object_loading::ObjectLoadingBundle;
    use object_model::ObjectType;
    use typename::TypeName;

    use super::CharacterSelectionSpawningSystem;
    use GameLoadingStatus;

    const ASSETS_MAP_FADE_NAME: &str = "fade";
    const ASSETS_CHAR_BAT_NAME: &str = "bat";

    lazy_static! {
        /// Slug of the "fade" map asset.
        static ref ASSETS_MAP_FADE_SLUG: AssetSlug = {
            AssetSlugBuilder::default()
                .namespace(ASSETS_TEST_DIR.to_string())
                .name(ASSETS_MAP_FADE_NAME.to_string())
                .build()
                .expect(&format!(
                    "Expected `{}/{}` asset slug to build.",
                    ASSETS_TEST_DIR,
                    ASSETS_MAP_FADE_NAME
                ))
        };
        /// Slug of the "bat" character asset.
        static ref ASSETS_CHAR_BAT_SLUG: AssetSlug = {
            AssetSlugBuilder::default()
                .namespace(ASSETS_TEST_DIR.to_string())
                .name(ASSETS_CHAR_BAT_NAME.to_string())
                .build()
                .expect(&format!(
                    "Expected `{}/{}` asset slug to build.",
                    ASSETS_TEST_DIR,
                    ASSETS_CHAR_BAT_NAME
                ))
        };
    }

    #[test]
    fn returns_if_characters_already_loaded() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("returns_if_characters_already_loaded", false)
                .with_setup(|world| {
                    let mut game_loading_status = GameLoadingStatus::new();
                    game_loading_status.characters_loaded = true;
                    world.add_resource(game_loading_status);

                    let char_entity = world.create_entity().build();
                    let mut objects = HashMap::new();
                    objects.insert(ObjectType::Character, vec![char_entity.clone()]);

                    world.add_resource(GameEntities::new(objects, Vec::new()));
                    world.add_resource(EffectReturn(char_entity));
                }).with_system_single(
                    CharacterSelectionSpawningSystem,
                    CharacterSelectionSpawningSystem::type_name(),
                    &[],
                ).with_assertion(|world| {
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
                }).run()
                .is_ok()
        );
    }

    // kcov-ignore-start
    #[test]
    #[ignore] // We can't test for panics because it poisons the test support Mutex
    #[should_panic]
    fn panics_when_map_selection_resource_not_present() {
        AmethystApplication::render_base("panics_when_map_selection_resource_not_present", false)
            .with_system(
                CharacterSelectionSpawningSystem,
                CharacterSelectionSpawningSystem::type_name(),
                &[],
            ).with_assertion(|_| {})
            .run()
            .ok();
    }
    // kcov-ignore-end

    #[test]
    fn spawns_characters_when_they_havent_been_spawned() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawns_characters_when_they_havent_been_spawned",
                false
            ).with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_state(|| LoadingState::new(
                Path::new(env!("CARGO_MANIFEST_DIR")).join(ASSETS),
                Box::new(EmptyState)
            )).with_setup(|world| {
                let first_map_handle = world
                    .read_resource::<MapAssets>()
                    .get(&ASSETS_MAP_FADE_SLUG)
                    .expect(&format!(
                        "Expected `{}` map to be loaded.",
                        *ASSETS_MAP_FADE_SLUG
                    )).clone();
                let map_selection =
                    MapSelection::new(MapSelectionStatus::Confirmed, Some(first_map_handle));

                world.add_resource(map_selection);
            }).with_setup(|world| {
                let mut character_selections = CharacterSelections::default();
                character_selections
                    .selections
                    .insert(0, ASSETS_CHAR_BAT_SLUG.clone());
                world.add_resource(character_selections);
            }).with_system_single(
                CharacterSelectionSpawningSystem,
                CharacterSelectionSpawningSystem::type_name(),
                &[],
            ).with_assertion(|world| {
                assert!(
                    !world
                        .read_resource::<GameEntities>()
                        .objects
                        .get(&ObjectType::Character)
                        .expect("Expected `ObjectType::Character` key in `GameEntities`.")
                        .is_empty()
                );
                assert!(world.read_resource::<GameLoadingStatus>().characters_loaded);
            }).run()
            .is_ok()
        );
    }
}

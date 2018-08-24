use amethyst::{assets::AssetStorage, ecs::prelude::*};
use character_selection::CharacterSelection;
use game_input::InputControlled;
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Kinematics, Position, Velocity},
    loaded::{Character, CharacterHandle},
    ObjectType,
};

use CharacterComponentStorages;
use CharacterEntitySpawner;
use ObjectComponentStorages;

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSpawningSystem;

type CharacterSelectionSpawningSystemData<'s> = (
    Read<'s, CharacterSelection>,
    Read<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    Entities<'s>,
    Read<'s, Vec<CharacterHandle>>,
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
            character_selection,
            map_selection,
            loaded_maps,
            entities,
            loaded_character_handles,
            loaded_characters,
            mut character_component_storages,
            mut object_component_storages,
            mut game_entities,
        ): Self::SystemData,
    ) {
        if let Some(characters) = game_entities.objects.get(&ObjectType::Character) {
            if !characters.is_empty() {
                // Already populated
                return;
            }
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

        let object_spawning_resources =
            (&*entities, &*loaded_character_handles, &*loaded_characters);

        let character_entities = character_selection
            .iter()
            .map(|(controller_id, character_index)| {
                (InputControlled::new(*controller_id), *character_index)
            }).map(|(input_controlled, character_index)| {
                CharacterEntitySpawner::spawn_system(
                    &object_spawning_resources,
                    &mut character_component_storages,
                    &mut object_component_storages,
                    kinematics,
                    character_index,
                    input_controlled,
                )
            }).collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);
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
    use character_selection::CharacterSelection;
    use game_model::play::GameEntities;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::MapHandle;
    use map_selection::{MapSelection, MapSelectionStatus};
    use object_loading::ObjectLoadingBundle;
    use object_model::ObjectType;
    use typename::TypeName;

    use super::CharacterSelectionSpawningSystem;

    #[test]
    fn returns_if_characters_already_populated() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("returns_if_characters_already_populated", false)
                .with_setup(|world| {
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
                    .read_resource::<Vec<MapHandle>>()
                    // TODO: <https://gitlab.com/azriel91/autexousious/issues/57>
                    .get(1) // assets/test/map/fade
                    .expect("Expected at least one map to be loaded.")
                    .clone();
                let map_selection =
                    MapSelection::new(MapSelectionStatus::Confirmed, Some(first_map_handle));

                world.add_resource(map_selection);
            }).with_setup(|world| {
                let mut character_selection = CharacterSelection::new();
                character_selection.insert(0, 0);
                world.add_resource(character_selection);
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
            }).run()
            .is_ok()
        );
    }
}

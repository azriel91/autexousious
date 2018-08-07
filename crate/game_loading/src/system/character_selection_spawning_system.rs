use amethyst::{assets::AssetStorage, ecs::prelude::*};
use character_selection::{CharacterEntityControl, CharacterSelection};
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Kinematics, Position, Velocity},
    loaded::{Character, CharacterHandle},
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
        ): Self::SystemData,
    ) {
        // Read map to determine bounds where the characters can be spawned.
        let map_handle = map_selection
            .map_handle
            .as_ref()
            .expect("Expected map to be selected.");

        let (width, height) = {
            loaded_maps
                .get(map_handle)
                .map(|map| {
                    let bounds = &map.definition.header.bounds;
                    (bounds.width as f32, bounds.height as f32)
                }).expect("Expected map to be loaded.")
        };

        // This `Position` moves the entity to the middle of a "screen wide" map.
        let position = Position::new(width / 2., height / 2., 0.);
        let kinematics = Kinematics::new(position, Velocity::default());

        let object_spawning_resources =
            (&*entities, &*loaded_character_handles, &*loaded_characters);

        character_selection
            .iter()
            .map(|(controller_id, character_index)| {
                (
                    CharacterEntityControl::new(*controller_id),
                    *character_index,
                )
            }).for_each(|(character_entity_control, character_index)| {
                CharacterEntitySpawner::spawn_system(
                    &object_spawning_resources,
                    &mut character_component_storages,
                    &mut object_component_storages,
                    kinematics,
                    character_index,
                    character_entity_control,
                );
            });
    }
}

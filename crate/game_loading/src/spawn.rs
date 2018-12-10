pub use self::{
    character_component_storages::CharacterComponentStorages,
    character_entity_spawner::CharacterEntitySpawner,
    map_layer_component_storages::MapLayerComponentStorages,
    map_layer_entity_spawner::MapLayerEntitySpawner,
    map_spawning_resources::MapSpawningResources,
    object_animation_storages::{
        BodyAcs, InteractionAcs, ObjectAnimationStorages, SpriteRenderAcs,
    },
    object_component_storages::ObjectComponentStorages,
    object_spawning_resources::ObjectSpawningResources,
};

mod character_component_storages;
mod character_entity_spawner;
mod map_layer_component_storages;
mod map_layer_entity_spawner;
mod map_spawning_resources;
mod object_animation_storages;
mod object_component_storages;
mod object_spawning_resources;

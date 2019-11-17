#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides runtime logic for `ui_label` types.

pub use self::{
    ui_sprite_label_component_storages::UiSpriteLabelComponentStorages,
    ui_sprite_label_entity_spawner::UiSpriteLabelEntitySpawner,
    ui_sprite_label_spawning_resources::UiSpriteLabelSpawningResources,
};

mod ui_sprite_label_component_storages;
mod ui_sprite_label_entity_spawner;
mod ui_sprite_label_spawning_resources;

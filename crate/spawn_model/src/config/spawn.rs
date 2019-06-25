use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::config::AssetSlug;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Specifies an object to spawn.
#[derive(Clone, Component, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct Spawn {
    /// Slug of the game object to spawn.
    #[serde(
        serialize_with = "AssetSlug::serialize_str",
        deserialize_with = "AssetSlug::deserialize_str"
    )]
    pub object: AssetSlug,
    /// `Position` that the spawned object begins with, relative to its parent.
    #[serde(default)]
    pub position: Position<i32>,
    /// `Velocity` that the spawned object begins with, relative to its parent.
    #[serde(default)]
    pub velocity: Velocity<i32>,
}

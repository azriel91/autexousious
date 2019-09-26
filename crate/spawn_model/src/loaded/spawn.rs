use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::loaded::AssetId;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use sequence_model::loaded::SequenceId;
use specs_derive::Component;

/// Specifies an object to spawn.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct Spawn {
    /// Asset ID of the game object to spawn.
    pub object: AssetId,
    /// `Position` that the spawned object begins with, relative to its parent.
    pub position: Position<f32>,
    /// `Velocity` that the spawned object begins with, relative to its parent.
    pub velocity: Velocity<f32>,
    /// `SequenceId` that the spawned object should begin with.
    pub sequence_id: SequenceId,
}

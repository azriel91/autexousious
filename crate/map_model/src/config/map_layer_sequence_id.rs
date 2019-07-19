use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derivative::Derivative;
use sequence_model::config::SequenceId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

/// `MapLayer` sequence IDs.
///
/// This is simply here as a type for the `SequenceEndTransition` component for map layers.
///
/// See `MapLayerEntitySpawner`.
#[derive(
    Clone,
    Copy,
    Debug,
    Derivative,
    Deserialize,
    Display,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    TypeName,
)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MapLayerSequenceId {
    /// Default sequence ID, unused.
    #[derivative(Default)]
    Layer,
}

impl Component for MapLayerSequenceId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl SequenceId for MapLayerSequenceId {}

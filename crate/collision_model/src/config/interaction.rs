use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;

use crate::config::CollisionMode;

/// Effects of one object on another
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(rename_all = "snake_case")]
pub struct Interaction {
    /// Effect behaviour of the collision -- impact, continuous, and so on.
    #[serde(default)]
    pub mode: CollisionMode,
    /// Effect volume.
    pub bounds: Vec<Volume>,
    /// Amount of health points (HP) to subtract on collision.
    #[serde(default)]
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    #[serde(default)]
    pub sp_damage: u32,
    /// Whether this will hit multiple objects. Defaults to `false`.
    #[serde(default)]
    pub multiple: bool,
}

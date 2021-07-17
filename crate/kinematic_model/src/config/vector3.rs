use derive_new::new;
use serde::{Deserialize, Serialize};

/// Type to represent serialized form of kinematic types.
///
/// This allows the axes to be named when specifying values, e.g. `{ x: -1, y:
/// 2, z: 3 }`. It also allows unspecified values to be defaulted.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Vector3<S>
where
    S: Default + Send + Sync + 'static,
{
    /// X coordinate.
    pub x: S,
    /// Y coordinate.
    pub y: S,
    /// Z coordinate.
    pub z: S,
}

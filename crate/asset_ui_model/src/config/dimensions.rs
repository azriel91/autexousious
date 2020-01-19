use serde::{Deserialize, Serialize};

/// Width and height of a UI element.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dimensions {
    /// Width of the element.
    pub w: u32,
    /// Height of the element.
    pub h: u32,
}

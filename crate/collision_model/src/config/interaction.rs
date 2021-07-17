#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;

use crate::config::InteractionKind;

/// Effects of one object on another
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct Interaction {
    /// Type of collision -- hit, picking weapon, grabbing, and so on.
    #[serde(flatten)]
    pub kind: InteractionKind,
    /// Effect volume.
    pub bounds: Vec<Volume>,
    /// Whether this will collide with multiple objects. Defaults to `false`.
    #[serde(default)]
    pub multiple: bool,
}

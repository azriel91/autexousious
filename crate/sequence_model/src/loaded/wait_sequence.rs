use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};

use crate::{component_sequence, config::Wait};

/// Sequence of `Wait` values.
#[component_sequence(Wait)]
pub struct WaitSequence;

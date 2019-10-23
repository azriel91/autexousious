use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::LayerPosition;

/// Positions of each layer in the background.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct LayerPositions(pub Vec<LayerPosition>);

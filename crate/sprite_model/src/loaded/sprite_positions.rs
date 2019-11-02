use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::SpritePosition;

/// Positions of each sprite sequence in the background.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct SpritePositions(pub Vec<SpritePosition>);

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::UiSpriteLabel;

/// Newtype for `Vec<UiSpriteLabel>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct UiSpriteLabels(pub Vec<UiSpriteLabel>);

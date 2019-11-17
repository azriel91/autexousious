use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::UiSpriteLabel;

/// Newtype for `Vec<UiSpriteLabel>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiSpriteLabels(pub Vec<UiSpriteLabel>);

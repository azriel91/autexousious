use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::UiLabel;

/// Newtype for `Vec<UiLabel>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct UiLabels(pub Vec<UiLabel>);

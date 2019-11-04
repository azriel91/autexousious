use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::UiMenuItem;

/// Newtype for `Vec<UiMenuItem>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiMenuItems<I>(pub Vec<UiMenuItem<I>>);

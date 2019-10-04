use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Interaction;

/// Effects on other objects.
#[derive(
    Asset, Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Eq, Serialize, new,
)]
pub struct Interactions(
    /// Backing vector of `Interaction`s.
    #[serde(default)]
    pub Vec<Interaction>,
);

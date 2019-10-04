use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;

/// Hittable volumes of an interactable object.
#[derive(
    Asset, Clone, Debug, Default, Deref, DerefMut, Deserialize, Hash, PartialEq, Eq, Serialize, new,
)]
pub struct Body(
    /// Backing vector of `Volume`s.
    #[serde(default)]
    pub Vec<Volume>,
);

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::BackgroundLayer;

/// Background layers for an asset.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct BackgroundLayers(pub Vec<BackgroundLayer>);

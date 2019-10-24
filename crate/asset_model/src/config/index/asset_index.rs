use std::collections::HashMap;

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::{index::AssetRecord, AssetType};

/// Index of all assets.
#[derive(Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AssetIndex(pub HashMap<AssetType, Vec<AssetRecord>>);

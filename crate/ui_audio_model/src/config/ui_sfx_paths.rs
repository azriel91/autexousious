use std::{collections::HashMap, path::PathBuf};

use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::UiSfxId;

/// Map of `UiSfxId` to the path of the SFX file.
#[derive(Asset, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, transparent)]
pub struct UiSfxPaths(HashMap<UiSfxId, PathBuf>);

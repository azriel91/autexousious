use derive_deref::{Deref, DerefMut};
use derive_new::new;
use indexmap::IndexMap;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;

use crate::config::AswPortraitName;

/// Portraits available to an `AssetSelectionWidget`.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct AswPortraits(pub IndexMap<AswPortraitName, SequenceNameString<SpriteSequenceName>>);

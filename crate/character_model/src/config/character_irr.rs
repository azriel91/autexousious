use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::CharacterIrrPart;

/// Character input reaction requirement.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterIrr(pub Vec<CharacterIrrPart>);

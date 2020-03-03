use std::collections::HashMap;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::play::{Session, SessionCode};

/// Sessions (`HashMap<SessionCode, Session>` newtype).
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct Sessions(pub HashMap<SessionCode, Session>);

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::PositionInit;

/// Position initializers for an asset.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct PositionInits(pub Vec<PositionInit>);

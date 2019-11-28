use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::ItemId;

/// Newtype for `Vec<ItemId>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct ItemIds(pub Vec<ItemId>);

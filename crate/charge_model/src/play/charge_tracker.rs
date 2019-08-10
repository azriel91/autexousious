use std::marker::PhantomData;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ChargePoints;

/// Tracker for charge points stored for the given `VariantStruct`;
#[derive(Clone, Copy, Debug, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct ChargeTracker<T>(pub ChargePoints, pub PhantomData<T>);

use derive_deref::{Deref, DerefMut};
use object_loading::ObjectLoadingStatus;
use typename_derive::TypeName;

/// `ObjectLoadingStatus` newtype for `Energy`s.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, PartialEq, Eq, TypeName)]
pub struct EnergyLoadingStatus(pub ObjectLoadingStatus);

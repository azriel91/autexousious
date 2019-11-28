use std::fmt::Debug;

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::UiMenuItem;

/// Newtype for `Vec<UiMenuItem>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct UiMenuItems<I>(pub Vec<UiMenuItem<I>>)
where
    I: Copy + Debug + PartialEq + Send + Sync + 'static;

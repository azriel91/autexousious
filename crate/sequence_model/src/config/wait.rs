use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use amethyst::ecs::{storage::VecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Number of ticks to stay on the current frame before switching to the next frame.
#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Display,
    From,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    new,
)]
#[storage(VecStorage)]
pub struct Wait(pub u32);

impl Add<u32> for Wait {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        Wait(self.0 + other)
    }
}

impl AddAssign<u32> for Wait {
    fn add_assign(&mut self, other: u32) {
        *self = Wait(self.0 + other);
    }
}

impl Sub<u32> for Wait {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        Wait(self.0 - other)
    }
}

impl SubAssign<u32> for Wait {
    fn sub_assign(&mut self, other: u32) {
        *self = Wait(self.0 - other);
    }
}

impl PartialOrd<u32> for Wait {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl PartialEq<u32> for Wait {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

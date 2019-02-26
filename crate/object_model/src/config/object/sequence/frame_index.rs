use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derive_deref::{Deref, DerefMut};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Number of ticks to stay on the current frame before switching to the next frame.
#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Clone,
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
pub struct FrameIndex(pub usize);

impl Component for FrameIndex {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Add<usize> for FrameIndex {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        FrameIndex(self.0 + other)
    }
}

impl AddAssign<usize> for FrameIndex {
    fn add_assign(&mut self, other: usize) {
        *self = FrameIndex(self.0 + other);
    }
}

impl Sub<usize> for FrameIndex {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        FrameIndex(self.0 - other)
    }
}

impl SubAssign<usize> for FrameIndex {
    fn sub_assign(&mut self, other: usize) {
        *self = FrameIndex(self.0 - other);
    }
}

impl PartialOrd<usize> for FrameIndex {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl PartialEq<usize> for FrameIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

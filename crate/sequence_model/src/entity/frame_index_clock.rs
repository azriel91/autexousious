use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use logic_clock::LogicClock;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Logic clock to track frame index of an object sequence.
#[derive(
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    From,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    new,
)]
pub struct FrameIndexClock(pub LogicClock);

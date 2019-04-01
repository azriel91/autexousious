use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use logic_clock_derive::logic_clock;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Logic clock to track frame index of an object sequence.
#[logic_clock]
pub struct FrameIndexClock;

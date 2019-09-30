use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};

/// Logic clock to track frame index of an object sequence.
#[logic_clock]
pub struct FrameIndexClock;

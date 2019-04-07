use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Logic clock to track wait value during a frame.
#[logic_clock]
pub struct FrameWaitClock;

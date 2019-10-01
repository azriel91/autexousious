use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};

/// Number of ticks a button must be held before `Charging` begins.
#[logic_clock]
pub struct ChargeBeginDelayClock;

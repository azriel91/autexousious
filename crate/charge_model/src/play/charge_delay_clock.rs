use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Number of ticks between charge accumulation.
#[logic_clock]
pub struct ChargeDelayClock;

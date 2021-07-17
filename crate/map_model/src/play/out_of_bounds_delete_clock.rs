use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};

/// Logic clock to track objects that should be deleted when out of bounds after
/// a delay.
#[logic_clock]
pub struct OutOfBoundsDeleteClock;

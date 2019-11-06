use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};

/// Logic clock delay game play end transition from happening.
#[logic_clock]
pub struct GamePlayEndTransitionDelayClock;

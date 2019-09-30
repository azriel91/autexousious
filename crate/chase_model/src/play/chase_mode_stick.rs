use amethyst::ecs::{storage::NullStorage, Component};
use derive_new::new;

/// Component indicating the chaser should stick to the target object.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, new)]
#[storage(NullStorage)]
pub struct ChaseModeStick;

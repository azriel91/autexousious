use amethyst::ecs::{storage::NullStorage, Component};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Indicates that a sequence should restart when it reaches the end.
#[derive(Debug, Default, Component, Deserialize, Serialize)]
#[storage(NullStorage)]
pub struct Repeat;

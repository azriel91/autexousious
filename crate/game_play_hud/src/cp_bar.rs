use amethyst::ecs::{storage::NullStorage, Component};

/// Tag component for charge point bars.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct CpBar;

use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Active / Idle status of a UI widget.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum WidgetStatus {
    /// Widget is not active.
    #[derivative(Default)]
    Idle,
    /// Widget is active.
    Active,
}

impl Component for WidgetStatus {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

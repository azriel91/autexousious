use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use specs_derive::Component;

use crate::loaded::{ActionHold, ActionPress, ActionRelease, AxisTransition, FallbackTransition};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq, new)]
#[storage(VecStorage)]
pub enum ControlTransition {
    /// Transition to a specified sequence on control input press event.
    ActionPress(ActionPress),
    /// Transition to a specified sequence on control input enabled state.
    ActionHold(ActionHold),
    /// Transition to a specified sequence on control input release event.
    ActionRelease(ActionRelease),
    /// Transition to a specified sequence on axis input press event.
    AxisPress(AxisTransition),
    /// Transition to a specified sequence on axis input state.
    AxisHold(AxisTransition),
    /// Transition to a specified sequence on axis input press event.
    AxisRelease(AxisTransition),
    /// Transition to a specified fallback sequence.
    Fallback(FallbackTransition),
}

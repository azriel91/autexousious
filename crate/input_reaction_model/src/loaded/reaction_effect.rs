use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;

use crate::loaded::{AxisTransition, FallbackTransition, ReactionEffectButton, ReactionEffectData};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub enum ReactionEffect {
    /// Transition to a specified sequence on control input press event.
    ActionPress(ReactionEffectData),
    /// Transition to a specified sequence on control input enabled state.
    ActionHold(ReactionEffectData),
    /// Transition to a specified sequence on control input release event.
    ActionRelease(ReactionEffectData),
    /// Transition to a specified sequence on axis input press event.
    AxisPress(AxisTransition),
    /// Transition to a specified sequence on axis input state.
    AxisHold(AxisTransition),
    /// Transition to a specified sequence on axis input press event.
    AxisRelease(AxisTransition),
    /// Transition to a specified fallback sequence.
    Fallback(FallbackTransition),
    /// Transition to a specified sequence on device button press event.
    ButtonPress(ReactionEffectButton),
}

impl AsRef<ReactionEffect> for ReactionEffect {
    fn as_ref(&self) -> &ReactionEffect {
        self
    }
}

impl AsRef<()> for ReactionEffect {
    fn as_ref(&self) -> &() {
        &()
    }
}

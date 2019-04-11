use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SequenceId;

/// Sequence ID to transition to when a `ControlAction` is interacted with.
///
/// This type is parallel to the `loaded` model to make it more ergonomic for creators to write
/// configuration.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// Sequence ID to transition to when `Defend` is pressed.
    PressDefend(SeqId),
    /// Sequence ID to transition to when `Jump` is pressed.
    PressJump(SeqId),
    /// Sequence ID to transition to when `Attack` is pressed.
    PressAttack(SeqId),
    /// Sequence ID to transition to when `Special` is pressed.
    PressSpecial(SeqId),
    /// Sequence ID to transition to when `Defend` is held.
    HoldDefend(SeqId),
    /// Sequence ID to transition to when `Jump` is held.
    HoldJump(SeqId),
    /// Sequence ID to transition to when `Attack` is held.
    HoldAttack(SeqId),
    /// Sequence ID to transition to when `Special` is held.
    HoldSpecial(SeqId),
    /// Sequence ID to transition to when `Defend` is released.
    ReleaseDefend(SeqId),
    /// Sequence ID to transition to when `Jump` is released.
    ReleaseJump(SeqId),
    /// Sequence ID to transition to when `Attack` is released.
    ReleaseAttack(SeqId),
    /// Sequence ID to transition to when `Special` is released.
    ReleaseSpecial(SeqId),
}

use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SequenceId;

/// Sequence ID to transition to when a `ControlAction` is pressed or held.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct ControlActionTransitions<SeqId>
where
    SeqId: SequenceId,
{
    /// Sequence ID to transition to when `Defend` is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_defend: Option<SeqId>,
    /// Sequence ID to transition to when `Jump` is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_jump: Option<SeqId>,
    /// Sequence ID to transition to when `Attack` is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_attack: Option<SeqId>,
    /// Sequence ID to transition to when `Special` is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_special: Option<SeqId>,
    /// Sequence ID to transition to when `Defend` is held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_defend: Option<SeqId>,
    /// Sequence ID to transition to when `Jump` is held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_jump: Option<SeqId>,
    /// Sequence ID to transition to when `Attack` is held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_attack: Option<SeqId>,
    /// Sequence ID to transition to when `Special` is held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_special: Option<SeqId>,
}

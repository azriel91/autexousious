use derivative::Derivative;
use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::{BasicIrr, ButtonInputReactionN, InputReaction};

/// Sequence to transition to when a `ControlAction` is pressed, held, or
/// released.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize, new)]
#[derivative(Default)]
#[serde(deny_unknown_fields)]
pub struct InputReactions<SeqName, IRR = BasicIrr>
where
    SeqName: SequenceName,
    IRR: Default,
{
    /// Sequence to transition to when `Defend` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_defend: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Jump` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_jump: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Attack` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_attack: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Special` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_special: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Defend` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_defend: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Jump` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_jump: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Attack` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_attack: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Special` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_special: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Defend` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_defend: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Jump` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_jump: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Attack` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_attack: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when `Special` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_special: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when X axis input is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_x: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when X axis input is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_x: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when X axis input is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_x: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when Z axis input is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_z: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when Z axis input is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_z: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when Z axis input is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_z: Option<InputReaction<SeqName, IRR>>,
    /// Fallback sequence to transition to.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<InputReaction<SeqName, IRR>>,
    /// Sequence to transition to when a keyboard `Button` is pressed.
    ///
    /// This is not intended to be used in the game model, but rather for the
    /// control settings UI to provide feedback when a device key is
    /// pressed. Device may be a keyboard, mouse, or controller.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_button: Option<ButtonInputReactionN<SeqName, IRR>>,
}

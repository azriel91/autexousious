use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ControlTransition, SequenceName};

/// Sequence to transition to when a `ControlAction` is pressed, held, or released.
#[derive(Clone, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[derivative(Default)]
#[serde(deny_unknown_fields)]
pub struct InputReactions<SeqName, Req = ()>
where
    SeqName: SequenceName,
    Req: Default,
{
    /// Sequence to transition to when `Defend` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_defend: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Jump` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_jump: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Attack` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_attack: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Special` is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_special: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Defend` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_defend: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Jump` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_jump: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Attack` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_attack: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Special` is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_special: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Defend` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_defend: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Jump` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_jump: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Attack` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_attack: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when `Special` is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_special: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when X axis input is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_x: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when X axis input is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_x: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when X axis input is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_x: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when Z axis input is pressed.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_z: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when Z axis input is held.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_z: Option<ControlTransition<SeqName, Req>>,
    /// Sequence to transition to when Z axis input is released.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_z: Option<ControlTransition<SeqName, Req>>,
    /// Fallback sequence to transition to.
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ControlTransition<SeqName, Req>>,
}

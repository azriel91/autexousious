use derive_new::new;
use sequence_model::config::ControlTransition;
use serde::{Deserialize, Serialize};

use crate::config::{CharacterSequenceId, ControlTransitionRequirement};

/// Sequence ID to transition to when a `ControlAction` is pressed or held.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct CharacterControlTransition {
    /// Action and target sequence.
    #[serde(flatten)]
    pub control_transition: ControlTransition<CharacterSequenceId>,
    /// Conditions for this transition to happen.
    #[serde(default, flatten)]
    pub requirement: ControlTransitionRequirement,
}

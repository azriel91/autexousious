use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ControlTransitionMultiple, ControlTransitionSingle, SequenceId};

/// Variants of how a `ControlTransition` may be specified.
///
/// This is primarily to make it more ergonomic for users to specify different kinds of values in
/// configuration.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum ControlTransition<SeqId, Req>
where
    SeqId: SequenceId,
    Req: Default,
{
    /// Transition that only has a sequence ID.
    ///
    /// ```yaml
    /// press_attack: "seq_id"
    /// ```
    SequenceId(SeqId),
    /// Transition has a sequence ID and extra fields.
    ///
    /// ```yaml
    /// press_attack: { next: "seq_id", extra_0: 0, extra_1: "0" }
    /// ```
    Single(ControlTransitionSingle<SeqId, Req>),
    /// Multiple transitions with sequence ID and extra fields.
    ///
    /// ```yaml
    /// press_attack:
    ///   - { next: "seq_id_0", extra_0: 0, extra_1: "0" }
    ///   - { next: "seq_id_1", extra_0: 1, extra_1: "1" }
    /// ```
    Multiple(ControlTransitionMultiple<SeqId, Req>),
}

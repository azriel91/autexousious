use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{
    ControlTransitionMultiple, ControlTransitionSingle, SequenceName, SequenceNameString,
};

/// Variants of how a `ControlTransition` may be specified.
///
/// This is primarily to make it more ergonomic for users to specify different kinds of values in
/// configuration.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum ControlTransition<SeqName, Req>
where
    SeqName: SequenceName,
    Req: Default,
{
    /// Transition that only has a sequence name.
    ///
    /// ```yaml
    /// press_attack: "sequence_name"
    /// ```
    SequenceNameString(SequenceNameString<SeqName>),
    /// Transition has a sequence name and extra fields.
    ///
    /// ```yaml
    /// press_attack: { next: "sequence_name", extra_0: 0, extra_1: "0" }
    /// ```
    Single(ControlTransitionSingle<SeqName, Req>),
    /// Multiple transitions with sequence name and extra fields.
    ///
    /// ```yaml
    /// press_attack:
    ///   - { next: "sequence_name_0", extra_0: 0, extra_1: "0" }
    ///   - { next: "sequence_name_1", extra_0: 1, extra_1: "1" }
    /// ```
    Multiple(ControlTransitionMultiple<SeqName, Req>),
}

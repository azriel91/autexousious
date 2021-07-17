use derive_new::new;
use sequence_model::config::{SequenceName, SequenceNameString};
use serde::{Deserialize, Serialize};

use crate::config::{InputReactionMultiple, InputReactionSingle};

/// Variants of how an `InputReaction` may be specified.
///
/// This is primarily to make it more ergonomic for users to specify different
/// kinds of values in configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum InputReaction<SeqName, IRR>
where
    SeqName: SequenceName,
    IRR: Default,
{
    /// Input reaction that simply transitions the sequence.
    ///
    /// ```yaml
    /// press_attack: "sequence_name"
    /// ```
    SequenceNameString(SequenceNameString<SeqName>),
    /// Input reaction that may transition the sequence and / or send events.
    ///
    /// ```yaml
    /// press_attack: { next: "sequence_name", extra_0: 0, extra_1: "0" }
    /// ```
    Single(InputReactionSingle<SeqName, IRR>),
    /// List of input reactions in decreasing priority order.
    ///
    /// ```yaml
    /// press_attack:
    ///   - { next: "sequence_name_0", extra_0: 0, extra_1: "0" }
    ///   - { next: "sequence_name_1", extra_0: 1, extra_1: "1" }
    /// ```
    Multiple(InputReactionMultiple<SeqName, IRR>),
}

impl<SeqName, IRR> AsRef<InputReaction<SeqName, IRR>> for InputReaction<SeqName, IRR>
where
    SeqName: SequenceName,
    IRR: Default,
{
    fn as_ref(&self) -> &InputReaction<SeqName, IRR> {
        self
    }
}

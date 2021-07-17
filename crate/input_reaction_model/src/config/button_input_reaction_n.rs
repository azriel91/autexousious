use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::{ButtonInputReaction, ButtonInputReactions};

/// Variants of how `ButtonInputReaction`s may be specified.
///
/// This is primarily to make it more ergonomic for users to specify different
/// kinds of values in configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum ButtonInputReactionN<SeqName, IRR>
where
    SeqName: SequenceName,
    IRR: Default,
{
    /// Single button input reaction.
    One(ButtonInputReaction<SeqName, IRR>),
    /// List of button input reactions in decreasing priority order.
    Many(ButtonInputReactions<SeqName, IRR>),
}

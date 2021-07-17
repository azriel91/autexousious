#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use asset_derive::Asset;
use charge_model::config::{ChargeDelay, ChargeLimit, ChargeRetentionMode, ChargeUseMode};
use derive_new::new;
use object_model::config::ObjectDefinition;
use serde::{Deserialize, Serialize};

use crate::config::CharacterSequence;

/// Contains all of the sequences for a `Character`.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<CharacterSequence>,
    /// Maximum charge this character can store.
    #[serde(default)]
    pub charge_limit: ChargeLimit,
    /// Number of ticks to wait between charge increments.
    #[serde(default)]
    pub charge_delay: ChargeDelay,
    /// Charge usage subtraction variants.
    #[serde(default)]
    pub charge_use_mode: ChargeUseMode,
    /// How charge is retained when no longer charging.
    #[serde(default)]
    pub charge_retention_mode: ChargeRetentionMode,
}

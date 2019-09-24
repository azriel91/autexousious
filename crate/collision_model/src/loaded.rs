//! Types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    asset_body_sequence_handles::AssetBodySequenceHandles,
    asset_interactions_sequence_handles::AssetInteractionsSequenceHandles,
    body_sequence::{BodySequence, BodySequenceHandle},
    body_sequence_handles::BodySequenceHandles,
    hit_transition::HitTransition,
    hitting_transition::HittingTransition,
    interactions_sequence::{InteractionsSequence, InteractionsSequenceHandle},
    interactions_sequence_handles::InteractionsSequenceHandles,
};

mod asset_body_sequence_handles;
mod asset_interactions_sequence_handles;
mod body_sequence;
mod body_sequence_handles;
mod hit_transition;
mod hitting_transition;
mod interactions_sequence;
mod interactions_sequence_handles;

//! Types representing collision configuration.

pub use self::{
    body::Body, impact::Impact, impact_repeat_delay::ImpactRepeatDelay, interaction::Interaction,
    interaction_kind::InteractionKind, interactions::Interactions,
};

mod body;
mod impact;
mod impact_repeat_delay;
mod interaction;
mod interaction_kind;
mod interactions;

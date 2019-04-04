//! Types representing collision configuration.

pub use self::{
    body::Body, hit_limit::HitLimit, impact::Impact, impact_repeat_delay::ImpactRepeatDelay,
    interaction::Interaction, interaction_kind::InteractionKind, interactions::Interactions,
};

mod body;
mod hit_limit;
mod impact;
mod impact_repeat_delay;
mod interaction;
mod interaction_kind;
mod interactions;

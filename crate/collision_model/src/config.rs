//! Types representing collision configuration.

pub use self::{
    body::Body, hit::Hit, hit_limit::HitLimit, hit_repeat_delay::HitRepeatDelay,
    interaction::Interaction, interaction_kind::InteractionKind, interactions::Interactions,
};

mod body;
mod hit;
mod hit_limit;
mod hit_repeat_delay;
mod interaction;
mod interaction_kind;
mod interactions;

//! Types representing collision configuration.

pub use self::{
    body::Body, collision_mode::CollisionMode, impact::Impact,
    impact_repeat_delay::ImpactRepeatDelay, interaction::Interaction, interactions::Interactions,
};

mod body;
mod collision_mode;
mod impact;
mod impact_repeat_delay;
mod interaction;
mod interactions;

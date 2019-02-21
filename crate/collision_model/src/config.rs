//! Types representing collision configuration.

pub use self::{
    body::Body, body_frame::BodyFrame, interaction::Interaction,
    interaction_frame::InteractionFrame, interactions::Interactions,
};

mod body;
mod body_frame;
mod interaction;
mod interaction_frame;
mod interactions;

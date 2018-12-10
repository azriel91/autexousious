//! Types representing collision configuration.

pub use self::{
    body_frame::BodyFrame, interaction::Interaction, interaction_frame::InteractionFrame,
};

mod body_frame;
mod interaction;
mod interaction_frame;

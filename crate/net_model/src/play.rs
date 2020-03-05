//! Data types used at runtime.

pub use self::{net_data::NetData, net_event_channel::NetEventChannel, net_message::NetMessage};

mod net_data;
mod net_event_channel;
mod net_message;

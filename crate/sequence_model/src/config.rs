//! Contains the types that represent the configuration on disk.

pub use self::{
    control_action_transitions::ControlActionTransitions, repeat::Repeat, sequence_id::SequenceId,
    wait::Wait,
};

mod control_action_transitions;
mod repeat;
mod sequence_id;
mod wait;

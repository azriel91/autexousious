//! Contains the types that represent the configuration on disk.

pub use self::{
    energy_definition::{EnergyDefinition, EnergyDefinitionHandle},
    energy_frame::EnergyFrame,
    energy_sequence::EnergySequence,
    energy_sequence_name::EnergySequenceName,
};

mod energy_definition;
mod energy_frame;
mod energy_sequence;
mod energy_sequence_name;

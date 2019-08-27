//! Contains the types that represent the configuration on disk.

pub use self::{
    test_object_definition::TestObjectDefinition, test_object_frame::TestObjectFrame,
    test_object_sequence::TestObjectSequence, test_object_sequence_name::TestObjectSequenceName,
};

mod test_object_definition;
mod test_object_frame;
mod test_object_sequence;
mod test_object_sequence_name;

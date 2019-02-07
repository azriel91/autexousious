use collision_model::loaded::BodySequence;

use crate::config::object::{Sequence, SequenceId};

/// Variants of component sequences of an object.
#[derive(Clone, Debug, PartialEq)]
pub enum ComponentSequence<SeqId>
where
    SeqId: SequenceId,
{
    /// Body (hurt boxes).
    BodySequence(BodySequence<Sequence<SeqId>>),
}

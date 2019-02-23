use collision_model::loaded::BodySequence;

/// Variants of component sequences of an object.
#[derive(Clone, Debug, PartialEq)]
pub enum ComponentSequence {
    /// Body (hurt boxes).
    BodySequence(BodySequence),
}

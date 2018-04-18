/// Types of in-game objects.
///
/// In-game objects are those that can be interacted with.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ObjectType {
    /// Player or AI controllable objects.
    Character,
}

impl ObjectType {
    /// Returns a snake_case `&str` for the object type.
    pub fn name(&self) -> &'static str {
        match *self {
            ObjectType::Character => "character",
        }
    }

    /// Returns a vector of the variants in this enum.
    pub fn variants() -> Vec<Self> {
        vec![ObjectType::Character]
    }
}

use entity::HealthPoints;

/// Indicates what fields of a `CharacterStatus` should be updated.
#[derive(Default, Debug, PartialEq, new)]
pub struct CharacterStatusUpdate {
    /// Health points.
    pub hp: Option<HealthPoints>,
}

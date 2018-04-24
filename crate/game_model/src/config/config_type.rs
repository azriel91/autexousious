/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are discovered.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConfigType {
    /// Configuration type for things that can be interacted with in-game.
    Object,
}

impl ConfigType {
    /// Returns a snake_case `&str` for the configuration type.
    pub fn name(&self) -> &'static str {
        match *self {
            ConfigType::Object => "object",
        }
    }

    /// Returns a vector of the variants in this enum.
    pub fn variants() -> Vec<Self> {
        vec![ConfigType::Object]
    }
}

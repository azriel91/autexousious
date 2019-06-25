use strum_macros::Display;

/// Enum representing asset slug fields.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum AssetSlugSegment {
    /// Namespace segment, e.g. `default`, `username`.
    Namespace,
    /// Name segment, e.g. `fireball`.
    Name,
}

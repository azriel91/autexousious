/// Style variants of fonts.
///
/// we tell Rust to represent this as a `u8` as Amethyst uses `usize`s as IDs when registering
/// resources in a `World`.
///
/// See:
///
/// * <https://stackoverflow.com/q/41648339/1576773>
/// * <https://doc.rust-lang.org/nomicon/other-reprs.html>
#[repr(usize)]
#[derive(Debug)]
pub enum FontVariant {
    /// For normal text.
    Regular = 0,
    /// For important text.
    Bold,
    /// For emphasized text.
    Italic,
    /// For important, emphasized text.
    BoldItalic,
}

impl From<FontVariant> for usize {
    fn from(variant: FontVariant) -> usize {
        match variant {
            FontVariant::Bold => 0,
            FontVariant::Italic => 1,
            FontVariant::BoldItalic => 2,
            FontVariant::Regular => 3,
        }
    }
}

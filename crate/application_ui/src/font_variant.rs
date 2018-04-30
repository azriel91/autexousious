#![allow(missing_debug_implementations)] // derived `EnumIter` does not implement Debug

/// Style variants of fonts.
#[warn(missing_docs)] // Pending <https://github.com/Peternator7/strum/issues/19>
#[derive(Debug, Display, Eq, EnumIter, Hash, PartialEq)]
pub enum FontVariant {
    /// For normal text.
    Regular,
    /// For important text.
    Bold,
    /// For emphasized text.
    Italic,
    /// For important, emphasized text.
    BoldItalic,
}

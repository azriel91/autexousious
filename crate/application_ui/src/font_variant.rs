/// Style variants of fonts.
///
/// we tell Rust to represent this as a `u8` as Amethyst uses `usize`s as IDs when registering
/// resources in a `World`.
///
/// See:
///
/// * <https://stackoverflow.com/q/41648339/1576773>
/// * <https://doc.rust-lang.org/nomicon/other-reprs.html>
#[repr(u8)]
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

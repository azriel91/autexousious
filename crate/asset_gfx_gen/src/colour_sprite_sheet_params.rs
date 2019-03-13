/// Parameters to generating a colour sprite sheet.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColourSpriteSheetParams {
    /// Individual sprite width.
    pub sprite_w: u32,
    /// Individual sprite height.
    pub sprite_h: u32,
    /// Whether there is a 1 pixel layer of padding pixels between each sprite.
    pub padded: bool,
    /// Number of rows of sprites (count vertically).
    pub row_count: usize,
    /// Number of rows of sprites (count horizontally).
    pub column_count: usize,
}

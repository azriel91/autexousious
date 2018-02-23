/// Holds the paths to font files under the `assets` directory.
#[derive(Debug, Deserialize)]
pub struct FontConfig {
    /// Regular text
    pub regular: String,
    /// Bold text
    pub bold: String,
    /// Italicized text
    pub italic: String,
    /// Bold and italicized text
    pub bold_italic: String,
}

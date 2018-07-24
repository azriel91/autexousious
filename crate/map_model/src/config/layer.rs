use config::Position;

/// An image layer on a map.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct Layer {
    /// Path to the image, relative to the map's directory.
    ///
    /// This is a `String` because Amethyst expects an `Into<String>` when loading an asset, and if
    /// we store a `PathBuf`, it would need to re-allocate another `String`.
    pub path: String,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
    /// Position of the image on the map.
    #[serde(default)]
    pub position: Position,
}

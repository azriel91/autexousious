/// Each asset's loading status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoadStage {
    /// The collective asset hasn't been loaded.
    New,
    /// Asset definition loading from disk.
    AssetDefinitionLoading,
    /// Sequence Name Strings mapping to Sequence IDs.
    IdMapping,
    /// Sprite definition loading from disk.
    SpritesDefinitionLoading,
    /// Texture loading from disk.
    TextureLoading,
    /// Sequence components loading from memory.
    SequenceComponentLoading,
    /// The collective asset is fully loaded.
    Complete,
}

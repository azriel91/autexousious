/// Each asset's loading status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoadStatus {
    /// The collective asset hasn't been loaded at all.
    New,
    /// Asset definition is loading.
    DefinitionLoading,
    /// Asset definition is loading.
    IdMapping,
    /// Sprite definition is loading.
    SpritesLoading,
    /// Textures are loading.
    TextureLoading,
    /// Sequence components are loading.
    SequenceComponentLoading,
    /// The collective asset is fully loaded.
    Complete,
}

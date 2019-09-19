/// Each asset's loading status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetLoadStatus {
    /// The collective asset hasn't been loaded at all.
    New,
    /// Asset definition is loading.
    DefinitionLoading,
    /// Sprite definition is loading.
    SpritesLoading,
    /// Sequence components are loading.
    SequenceComponentLoading,
    /// The collective asset is fully loaded.
    Complete,
}

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

impl LoadStage {
    /// Returns the next variant in the list of stages.
    ///
    /// Returns `None` if this is on the final stage.
    pub fn next(self) -> Option<LoadStage> {
        match self {
            Self::New => Some(Self::AssetDefinitionLoading),
            Self::AssetDefinitionLoading => Some(Self::IdMapping),
            Self::IdMapping => Some(Self::SpritesDefinitionLoading),
            Self::SpritesDefinitionLoading => Some(Self::TextureLoading),
            Self::TextureLoading => Some(Self::SequenceComponentLoading),
            Self::SequenceComponentLoading => Some(Self::Complete),
            Self::Complete => None,
        }
    }

    /// Returns the previous variant in the list of stages.
    ///
    /// Returns `None` if this is on the first stage.
    pub fn prev(self) -> Option<LoadStage> {
        match self {
            Self::New => None,
            Self::AssetDefinitionLoading => Some(Self::New),
            Self::IdMapping => Some(Self::AssetDefinitionLoading),
            Self::SpritesDefinitionLoading => Some(Self::IdMapping),
            Self::TextureLoading => Some(Self::SpritesDefinitionLoading),
            Self::SequenceComponentLoading => Some(Self::TextureLoading),
            Self::Complete => Some(Self::SequenceComponentLoading),
        }
    }
}

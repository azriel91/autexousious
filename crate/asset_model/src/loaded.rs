//! Types representing the loaded form of assets.

pub use self::{
    character_assets::CharacterAssets, map_assets::MapAssets, slug_and_handle::SlugAndHandle,
};

mod character_assets;
mod map_assets;
mod slug_and_handle;

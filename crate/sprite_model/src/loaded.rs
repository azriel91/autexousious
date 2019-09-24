//! Types that represent processed configuration.

pub use self::{
    asset_sprite_render_sequence_handles::AssetSpriteRenderSequenceHandles,
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
};

mod asset_sprite_render_sequence_handles;
mod sprite_render_sequence;
mod sprite_render_sequence_handles;

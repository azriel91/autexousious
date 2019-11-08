//! Types that represent processed configuration.

pub use self::{
    asset_sprite_positions::AssetSpritePositions,
    asset_sprite_render_sequence_handles::AssetSpriteRenderSequenceHandles,
    asset_tint_sequence_handles::AssetTintSequenceHandles,
    sprite_positions::SpritePositions,
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
    tint_sequence::{TintSequence, TintSequenceHandle},
    tint_sequence_handles::TintSequenceHandles,
};

mod asset_sprite_positions;
mod asset_sprite_render_sequence_handles;
mod asset_tint_sequence_handles;
mod sprite_positions;
mod sprite_render_sequence;
mod sprite_render_sequence_handles;
mod tint_sequence;
mod tint_sequence_handles;

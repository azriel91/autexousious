//! Types that represent processed configuration.

pub use self::{
    asset_sprite_positions::AssetSpritePositions,
    asset_sprite_render_sequence_handles::AssetSpriteRenderSequenceHandles,
    asset_tint::AssetTint,
    sprite_positions::SpritePositions,
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
};

mod asset_sprite_positions;
mod asset_sprite_render_sequence_handles;
mod asset_tint;
mod sprite_positions;
mod sprite_render_sequence;
mod sprite_render_sequence_handles;

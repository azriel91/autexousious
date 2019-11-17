//! Types that represent processed configuration.

pub use self::{
    asset_scale_sequence_handles::AssetScaleSequenceHandles,
    asset_sprite_render_sequence_handles::AssetSpriteRenderSequenceHandles,
    asset_tint_sequence_handles::AssetTintSequenceHandles,
    scale_sequence::{ScaleSequence, ScaleSequenceHandle},
    scale_sequence_handles::ScaleSequenceHandles,
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
    tint_sequence::{TintSequence, TintSequenceHandle},
    tint_sequence_handles::TintSequenceHandles,
};

mod asset_scale_sequence_handles;
mod asset_sprite_render_sequence_handles;
mod asset_tint_sequence_handles;
mod scale_sequence;
mod scale_sequence_handles;
mod sprite_render_sequence;
mod sprite_render_sequence_handles;
mod tint_sequence;
mod tint_sequence_handles;

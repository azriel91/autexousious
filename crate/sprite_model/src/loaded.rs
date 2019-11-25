//! Types that represent processed configuration.

pub use self::{
    scale_sequence::{ScaleSequence, ScaleSequenceHandle},
    scale_sequence_handles::ScaleSequenceHandles,
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
    tint_sequence::{TintSequence, TintSequenceHandle},
    tint_sequence_handles::TintSequenceHandles,
};

mod scale_sequence;
mod scale_sequence_handles;
mod sprite_render_sequence;
mod sprite_render_sequence_handles;
mod tint_sequence;
mod tint_sequence_handles;

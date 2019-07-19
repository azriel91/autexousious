//! Types that represent processed configuration.

pub use self::{
    sprite_render_sequence::{SpriteRenderSequence, SpriteRenderSequenceHandle},
    sprite_render_sequence_handles::SpriteRenderSequenceHandles,
};

mod sprite_render_sequence;
mod sprite_render_sequence_handles;

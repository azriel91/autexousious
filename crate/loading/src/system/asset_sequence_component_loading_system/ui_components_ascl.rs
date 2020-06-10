use input_reaction_model::loaded::InputReactionsSequenceHandles;
use sequence_model::loaded::{SequenceEndTransitions, WaitSequenceHandles};
use sprite_model::loaded::{
    ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles,
};

/// Common components for UI items.
#[derive(Clone, Debug)]
pub struct UiComponentsAscl {
    /// Sequence transition upon sequence end.
    pub sequence_end_transitions: SequenceEndTransitions,
    /// Sequence of `WaitSequenceHandle`s.
    pub wait_sequence_handles: WaitSequenceHandles,
    /// Sequence of `TintSequenceHandle`s.
    pub tint_sequence_handles: TintSequenceHandles,
    /// Sequence of `ScaleSequenceHandle`s.
    pub scale_sequence_handles: ScaleSequenceHandles,
    /// Sequence of `InputReactionsSequenceHandle`s.
    pub input_reactions_sequence_handles: InputReactionsSequenceHandles,
    /// Sequence of `SpriteRenderSequenceHandle`s.
    pub sprite_render_sequence_handles: Option<SpriteRenderSequenceHandles>,
}

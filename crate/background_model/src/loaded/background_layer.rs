use derive_new::new;
use sequence_model::loaded::SequenceId;

/// Background layer to spawn.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub struct BackgroundLayer {
    /// Sequence ID of the background layer.
    pub sequence_id: SequenceId,
}

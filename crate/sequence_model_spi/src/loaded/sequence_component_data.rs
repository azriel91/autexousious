use std::collections::HashMap;

use amethyst::ecs::Component;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use sequence_model_core::config::SequenceId;

/// Newtype for a component that changes when an entity's sequence changes.
///
/// This tracks the component that changes per sequence ID into a map, keyed by the sequence ID.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SequenceComponentData<SeqId, C>(
    /// The sequence component values, keyed by sequence ID.
    pub HashMap<SeqId, C>,
)
where
    SeqId: SequenceId,
    C: Component;

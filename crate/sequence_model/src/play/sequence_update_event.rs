use amethyst::ecs::Entity;

/// Event signalling a change in sequence or frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SequenceUpdateEvent {
    /// A new sequence is beginning.
    ///
    /// TODO: This variant does not hold the sequence ID as it previously was specific to different
    /// TODO: object types.
    /// TODO: Send sequence ID within event.
    SequenceBegin {
        /// Entity whose sequence changed.
        entity: Entity,
    },
    /// The next frame in the current sequence is beginning.
    FrameBegin {
        /// Entity whose sequence changed.
        entity: Entity,
        /// Current valid frame index.
        frame_index: usize,
    },
    /// The current sequence has ended.
    ///
    /// This means the last frame in the sequence has past its `Wait` time.
    SequenceEnd {
        /// Entity whose sequence ended.
        entity: Entity,
        /// Last valid frame index.
        frame_index: usize,
    },
}

impl SequenceUpdateEvent {
    /// Returns the entity this event corresponds to.
    pub fn entity(self) -> Entity {
        match self {
            SequenceUpdateEvent::SequenceBegin { entity }
            | SequenceUpdateEvent::FrameBegin { entity, .. }
            | SequenceUpdateEvent::SequenceEnd { entity, .. } => entity,
        }
    }

    /// Returns the last valid frame index of the sequence this event corresponds to.
    pub fn frame_index(self) -> usize {
        match self {
            SequenceUpdateEvent::SequenceBegin { .. } => 0,
            SequenceUpdateEvent::FrameBegin { frame_index, .. }
            | SequenceUpdateEvent::SequenceEnd { frame_index, .. } => frame_index,
        }
    }
}

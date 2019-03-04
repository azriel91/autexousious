use amethyst::ecs::Entity;

/// Event signalling a change in equence or frame.
#[derive(Clone, Debug, PartialEq)]
pub enum SequenceUpdateEvent {
    /// A new sequence is beginning.
    SequenceBegin {
        /// Entity whose sequence changed.
        entity: Entity,
    },
    /// The next frame in the current sequence is beginning.
    FrameBegin {
        /// Entity whose sequence changed.
        entity: Entity,
    },
    /// The current sequence has ended.
    ///
    /// This means the last frame in the sequence has past its `Wait` time.
    SequenceEnd {
        /// entity whose sequence ended.
        entity: Entity,
    },
}

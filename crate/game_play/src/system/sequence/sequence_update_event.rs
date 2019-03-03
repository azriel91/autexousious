use amethyst::ecs::Entity;

/// Event signalling a change in object sequence or frame.
#[derive(Clone, Debug, PartialEq)]
pub enum SequenceUpdateEvent {
    /// A new sequence is beginning.
    SequenceBegin {
        /// Object entity whose sequence changed.
        entity: Entity,
    },
    /// The next frame in the current sequence is beginning.
    FrameBegin {
        /// Object entity whose sequence changed.
        entity: Entity,
    },
}

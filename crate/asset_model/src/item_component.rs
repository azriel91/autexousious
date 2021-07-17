use amethyst::{
    ecs::{Component, Entity},
    shred::SystemData,
};

/// An independent part of an item that may be loaded from an asset, such as
/// `WaitSequenceHandles`.
pub trait ItemComponent<'s>: Component {
    /// `SystemData` needed when augmenting an entity with `Component`s.
    type SystemData: SystemData<'s>;

    /// Augments an entity with `Component`s given it has this `ItemComponent`.
    ///
    /// For example, given an entity has `WaitSequenceHandles`, it should be
    /// augmented with:
    ///
    /// * `FrameIndexClock`
    /// * `FrameWaitClock`
    fn augment(&self, _system_data: &mut Self::SystemData, _entity: Entity) {}
}

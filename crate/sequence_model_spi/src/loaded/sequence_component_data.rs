use amethyst::ecs::Component;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// Newtype for a component that changes when an entity's sequence changes.
///
/// This tracks the component that changes per sequence ID into a map, keyed by
/// the sequence ID.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SequenceComponentData<C>(
    /// The sequence component values, keyed by sequence ID.
    pub Vec<C>,
)
where
    C: Component;

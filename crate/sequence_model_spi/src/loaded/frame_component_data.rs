use amethyst::ecs::Component;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// Newtype for a specific component from all frames in a sequence.
///
/// A sequence contains frames, and each frame contains multiple components. This type is used to
/// collect all of the same component type into a single vector, so that each component data type is
/// stored separately from the others.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct FrameComponentData<C>(
    /// The chain of component values.
    pub Vec<C>,
)
where
    C: Component;

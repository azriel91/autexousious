use amethyst::ecs::Component;

/// Allows constraint clause query the component type.
pub trait ComponentSequenceExt {
    /// The ECS component type of this sequence.
    type Component: Component;
    /// Returns an owned version of the component.
    fn to_owned(component: &Self::Component) -> Self::Component;
}

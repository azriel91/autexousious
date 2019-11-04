use amethyst::ecs::Component;

/// Allows constraint clause query the component type.
pub trait ComponentDataExt {
    /// The ECS component type of this data.
    type Component: Component;

    /// Returns a new instance of this type.
    fn new(sequence: Vec<Self::Component>) -> Self;

    /// Returns an owned version of the component.
    fn to_owned(component: &Self::Component) -> Self::Component;
}

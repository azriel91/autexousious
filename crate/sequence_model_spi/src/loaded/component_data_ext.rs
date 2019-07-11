use amethyst::ecs::Component;

/// Allows constraint clause query the component type.
pub trait ComponentDataExt {
    /// The ECS component type of this data.
    type Component: Component;
    /// Returns an owned version of the component.
    fn to_owned(component: &Self::Component) -> Self::Component;
}

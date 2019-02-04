use amethyst::ecs::SystemData;

/// Workaround for `SystemData` while GAT is not yet available.
pub trait MapperSystemData<'s> {
    /// `SystemData` to read from the world.
    type SystemData: SystemData<'s>;
}

impl<'s, T> MapperSystemData<'s> for T
where
    T: SystemData<'s>,
{
    type SystemData = T;
}

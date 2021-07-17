use std::{any, marker::PhantomData};

use amethyst::{
    ecs::{Component, Entities, Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;

use crate::Last;

/// Tracks the value of an entity's component, and adds a `Last<T>` component
/// with that value.
///
/// The order that systems should be dispatched is:
///
/// 1. System that updates `T`
/// 2. System that reads `T` and `Last<T>`
/// 3. `LastTrackerSystem<T>`
///
/// The order is important, otherwise the value that is stored in `Last<T>` will
/// be exactly the same as `T`, providing no trackable detection.
///
/// This should be used conservatively if the tracked type is `Clone` and not
/// `Copy`, as the memory allocations can be a performance hit.
#[derive(Clone, Debug, Default, new)]
pub struct LastTrackerSystem<T>
where
    T: Component + Clone + Send + Sync,
{
    /// Stringified name of the `Component` tracked by this system.
    component_name: &'static str,
    /// Component tracked by this system.
    component: PhantomData<T>,
}

/// `LastTrackerSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct LastTrackerSystemData<'s, T>
where
    T: Clone + Component + Send + Sync,
{
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `T` components.
    #[derivative(Debug = "ignore")]
    pub components: ReadStorage<'s, T>,
    /// `Last<T>` components.
    #[derivative(Debug = "ignore")]
    pub components_lasts: WriteStorage<'s, Last<T>>,
}

impl<'s, T> LastTrackerSystem<T>
where
    T: Component + Clone + Send + Sync,
{
    /// Returns a String representing this system's name.
    pub fn system_name(&self) -> String {
        format!("{}<{}>", any::type_name::<Self>(), self.component_name)
    }
}

impl<'s, T> System<'s> for LastTrackerSystem<T>
where
    T: Component + Clone + Send + Sync,
{
    type SystemData = LastTrackerSystemData<'s, T>;

    fn run(
        &mut self,
        LastTrackerSystemData {
            entities,
            components,
            mut components_lasts,
        }: Self::SystemData,
    ) {
        (&*entities, &components)
            .join()
            .for_each(|(entity, component)| {
                components_lasts
                    .insert(entity, Last(component.clone()))
                    .unwrap_or_else(|_| {
                        // kcov-ignore-start
                        panic!(
                            "Failed to insert `{}<{}>` component.",
                            any::type_name::<Last::<T>>(),
                            self.component_name
                        )
                        // kcov-ignore-end
                    }); // kcov-ignore
            });
    }
}

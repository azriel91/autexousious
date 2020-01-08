use std::{any, marker::PhantomData};

use amethyst::{
    ecs::{LazyUpdate, Read, ReadExpect, System, World, WriteExpect},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;

use crate::Prev;

/// Tracks the value of a resource, and adds a `Prev<T>` resource with that value.
///
/// The order that systems should be dispatched is:
///
/// 1. System that updates `T`
/// 2. System that reads `T` and `Prev<T>`
/// 3. `PrevTrackerSystem<T>`
///
/// The order is important, otherwise the value that is stored in `Prev<T>` will be exactly the same
/// as `T`, providing no trackable detection.
///
/// This should be used conservatively if the tracked type is `Clone` and not `Copy`, as the memory
/// allocations can be a performance hit.
#[derive(Clone, Debug, Default, new)]
pub struct PrevTrackerSystem<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Stringified name of the `Component` tracked by this system.
    resource_name: &'static str,
    /// Component tracked by this system.
    resource: PhantomData<T>,
}

/// `PrevTrackerSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct PrevTrackerSystemData<'s, T>
where
    T: Clone + Send + Sync + 'static,
{
    /// `T` resource.
    #[derivative(Debug = "ignore")]
    pub resource: Option<ReadExpect<'s, T>>,
    /// `Prev<T>` resource.
    #[derivative(Debug = "ignore")]
    pub resource_prev: Option<WriteExpect<'s, Prev<T>>>,
    /// `LazyUpdate` resource.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
}

impl<'s, T> PrevTrackerSystem<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Returns a String representing this system's name.
    pub fn system_name(&self) -> String {
        format!("{}<{}>", any::type_name::<Self>(), self.resource_name)
    }
}

impl<'s, T> System<'s> for PrevTrackerSystem<T>
where
    T: Clone + Send + Sync + 'static,
{
    type SystemData = PrevTrackerSystemData<'s, T>;

    fn run(
        &mut self,
        PrevTrackerSystemData {
            resource,
            resource_prev,
            lazy_update,
        }: Self::SystemData,
    ) {
        if let Some(resource) = resource.as_ref() {
            let resource_prev_next = Prev::new((*resource).clone());
            if let Some(mut resource_prev) = resource_prev {
                *resource_prev = resource_prev_next;
            } else {
                lazy_update.exec_mut(move |world| world.insert(resource_prev_next));
            }
        }
    }
}

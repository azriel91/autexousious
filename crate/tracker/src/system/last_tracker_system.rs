use std::marker::PhantomData;

use amethyst::ecs::prelude::*;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;

use crate::Last;

/// Tracks the value of an entity's component, and adds a `Last<T>` component with that value.
///
/// The order that systems should be dispatched is:
///
/// 1. System that updates `T`
/// 2. System that reads `T` and `Last<T>`
/// 3. `LastTrackerSystem<T>`
///
/// The order is important, otherwise the value that is stored in `Last<T>` will be exactly the same
/// as `T`, providing no trackable detection.
///
/// This should be used conservatively if the tracked type is `Clone` and not `Copy`, as the memory
/// allocations can be a performance hit.
///
/// Implementation note: This uses the `named_type` crate instead of `typename` because we cannot
/// derive `TypeName` unless we add a `T: TypeName` bound.
#[derive(Clone, Debug, Default, NamedType, new)]
pub struct LastTrackerSystem<T: Component + Clone + Send + Sync> {
    /// Stringified name of the `Component` tracked by this system.
    component_name: &'static str,
    /// Component tracked by this system.
    component: PhantomData<T>,
}

type LastTrackerSystemData<'s, T> = (Entities<'s>, ReadStorage<'s, T>, WriteStorage<'s, Last<T>>);

impl<'s, T> LastTrackerSystem<T>
where
    T: Component + Clone + Send + Sync,
{
    /// Returns a String representing this system's name.
    pub fn system_name(&self) -> String {
        format!("{}<{}>", Self::type_name(), self.component_name)
    }
}

impl<'s, T> System<'s> for LastTrackerSystem<T>
where
    T: Component + Clone + Send + Sync,
{
    type SystemData = LastTrackerSystemData<'s, T>;

    fn run(&mut self, (entities, components, mut last_components): Self::SystemData) {
        (&*entities, &components)
            .join()
            .for_each(|(entity, component)| {
                last_components
                    .insert(entity, Last(component.clone()))
                    .unwrap_or_else(|_| {
                        // kcov-ignore-start
                        panic!(
                            "Failed to insert `{}<{}>` component.",
                            Last::<T>::type_name(),
                            self.component_name
                        )
                        // kcov-ignore-end
                    }); // kcov-ignore
            });
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::prelude::*;
    use amethyst_test::prelude::*;

    use super::{LastTrackerSystem, LastTrackerSystemData};
    use crate::Last;

    #[test]
    fn inserts_last_component_with_cloned_value() {
        let system = LastTrackerSystem::<TestComponent>::new(stringify!(TestComponent));
        let system_name = system.system_name();
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_setup(setup_components)
                .with_setup(|world| {
                    let entity = world.create_entity().with(TestComponent(123)).build();
                    world.insert(EffectReturn(entity));
                })
                .with_system_single(system.clone(), &system_name, &[])
                .with_assertion(|world| assert_last_value(world, 123))
                .with_effect(|world| {
                    let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();

                    let test_components = &mut world.write_storage::<TestComponent>();
                    let test_component = test_components
                        .get_mut(entity)
                        .expect("Entity should have a `TestComponent` component.");

                    test_component.0 = 456;
                })
                .with_system_single(system, &system_name, &[])
                .with_assertion(|world| assert_last_value(world, 456))
                .run()
                .is_ok()
        );
    }

    fn setup_components(world: &mut World) {
        LastTrackerSystemData::<TestComponent>::setup(&mut world.res);
    }

    fn assert_last_value(world: &mut World, value: i32) {
        let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();

        let last_test_components = world.read_storage::<Last<TestComponent>>();
        let last_test_component = last_test_components
            .get(entity)
            .expect("Entity should have a `Last<TestComponent>` component.");

        assert_eq!(value, (last_test_component.0).0);
    }

    #[derive(Clone)]
    struct TestComponent(pub i32);
    impl Component for TestComponent {
        type Storage = DenseVecStorage<Self>;
    }
}

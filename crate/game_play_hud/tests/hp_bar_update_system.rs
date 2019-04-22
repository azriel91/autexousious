// We cannot test `HpBarUpdateSystem` directly in its own module due to Rust's behaviour in the
// following scenario:
//
// 1. Crate `a_crate` provides `ComponentA`
// 2. Crate `b_crate` uses `ReadStorage<'_, ComponentA>`
// 3. `a_crate` uses `b_crate` in `[dev-dependencies]`, e.g. as part of `GameLoadingState`.
// 4. In `b_crate`, system setup will report:
//
// ```rust
// res.try_fetch::<MaskedStorage<game_play_hud::HpBar>>().is_some() = true
// ```
//
// 4. In `a_crate` in a test function, `world.read_storage::<crate::HpBar>` panics:
//
// ```
// thread '<unnamed>' panicked at 'Tried to fetch a resource of type "MaskedStorage<hp_bar::HpBar>",
// but the resource does not exist.
// ```
//
// `a_crate`'s compilation is separate from `a_crate`'s tests, and in the test, using
// `crate::ComponentA` gives us a different *version* of `ComponentA`.
//
// Paraphrased, `ComponentA` in the tests is different from `ComponentA` from `b_crate`.

use amethyst::{
    core::{Parent, Transform},
    ecs::{Entities, Entity, Join, ReadStorage},
    Error,
};
use application_test_support::{AutexousiousApplication, ObjectQueries};
use game_play_hud::{HpBar, HpBarUpdateSystem};
use object_model::{play::HealthPoints, ObjectType};

#[test]
fn sets_transform_x_and_scale() -> Result<(), Error> {
    AutexousiousApplication::game_base("sets_transform_x_and_scale", false)
        .with_setup(|world| {
            let char_entity = ObjectQueries::game_object_entity(world, ObjectType::Character);
            let hp_bar_entity = {
                let (entities, hp_bars, parents) = world.system_data::<(
                    Entities<'_>,
                    ReadStorage<'_, HpBar>,
                    ReadStorage<'_, Parent>,
                )>();
                (&entities, &hp_bars, &parents)
                    .join()
                    .find(|(_, _, parent)| parent.entity == char_entity)
                    .map(|(entity, _, _)| entity)
                    .expect("Expected `HpBar` entity to exist with character `Parent` entity.")
            };

            world.add_resource(hp_bar_entity);

            // Decrease character HP.
            let mut health_pointses = world.write_storage::<HealthPoints>();
            let health_points = health_pointses
                .get_mut(char_entity)
                .expect("Expected character to have `HealthPoints` component.");
            *health_points = HealthPoints(20);
        })
        .with_system_single(HpBarUpdateSystem::new(), "", &[])
        .with_assertion(|world| {
            let hp_bar_entity = *world.read_resource::<Entity>();

            let transforms = world.read_storage::<Transform>();
            let transform = transforms
                .get(hp_bar_entity)
                .expect("Expected hp bar to have `Transform` component.");

            // 100 - 20 = 80
            // -80 / 2  = -40
            assert_eq!(-40., transform.translation()[0]);
            assert_eq!(20., transform.scale()[0]);
        })
        .run()
}

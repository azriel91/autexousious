use amethyst::{
    assets::AssetStorage,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use object_model::{config::Mass, play::Grounding};
use typename_derive::TypeName;

/// Increases velocity of `Object`s that have `Mass` and are `Airborne`.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectGravitySystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectGravitySystemData<'s> {
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub maps: Read<'s, AssetStorage<Map>>,
    /// `Mass` components.
    #[derivative(Debug = "ignore")]
    pub masses: ReadStorage<'s, Mass>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> System<'s> for ObjectGravitySystem {
    type SystemData = ObjectGravitySystemData<'s>;

    fn run(
        &mut self,
        ObjectGravitySystemData {
            map_selection,
            maps,
            masses,
            mut positions,
            mut velocities,
            mut groundings,
        }: Self::SystemData,
    ) {
        let map_margins = {
            maps.get(map_selection.handle())
                .map(|map| map.margins)
                .expect("Expected map to be loaded.")
        };

        for (mass, position, velocity, grounding) in
            (&masses, &mut positions, &mut velocities, &mut groundings).join()
        {
            // X axis
            if position[0] < map_margins.left {
                position[0] = map_margins.left;
            } else if position[0] > map_margins.right {
                position[0] = map_margins.right;
            }

            // Y axis
            if position[1] > map_margins.bottom {
                velocity[1] -= **mass; // No gravity yet, so we just use `Mass` as weight.
                *grounding = Grounding::Airborne;

                if position[1] > map_margins.top {
                    position[1] = map_margins.top;
                }
            } else {
                position[1] = map_margins.bottom;
                velocity[1] = 0.;
                *grounding = Grounding::OnGround;
            }

            // Z axis
            if position[2] < map_margins.back {
                position[2] = map_margins.back;
            } else if position[2] > map_margins.front {
                position[2] = map_margins.front;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Join, ReadStorage, WriteStorage},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use kinematic_model::config::Position;
    use object_model::{config::Mass, play::Grounding};
    use typename::TypeName;

    use super::ObjectGravitySystem;

    #[test]
    fn keeps_character_within_lower_map_bounds() -> Result<(), Error> {
        run_test(
            |(position, _grounding, _mass)| {
                position[0] = -10.;
                position[1] = -10.;
                position[2] = -10.;
            },
            |(position, _grounding)| {
                assert_eq!(1., position[0]);

                // Map margins are shifted by z and depth. See
                // `map_model::loaded::Margins`
                assert_eq!(205., position[1]);
                assert_eq!(3., position[2]);
            },
        )
    }

    #[test]
    fn keeps_character_within_upper_map_bounds() -> Result<(), Error> {
        run_test(
            |(position, _grounding, _mass)| {
                position[0] = 2000.;
                position[1] = 2000.;
                position[2] = 2000.;
            },
            |(position, _grounding)| {
                assert_eq!(801., position[0]);

                // Map margins are shifted by z and depth. See
                // `map_model::loaded::Margins`
                assert_eq!(605., position[1]);
                assert_eq!(203., position[2]);
            },
        )
    }

    #[test]
    fn grounding_set_to_airborne_when_above_ground() -> Result<(), Error> {
        run_test(
            |(position, grounding, _mass)| {
                position[1] = 300.;
                *grounding = Grounding::OnGround;
            },
            |(_position, grounding)| {
                assert_eq!(Grounding::Airborne, *grounding);
            },
        )
    }

    #[test]
    fn grounding_set_to_on_ground_when_on_ground() -> Result<(), Error> {
        run_test(
            |(position, grounding, _mass)| {
                position[1] = 200.;
                *grounding = Grounding::Airborne;
            },
            |(_position, grounding)| {
                assert_eq!(Grounding::OnGround, *grounding);
            },
        )
    }

    fn run_test(
        setup_fn: fn((&mut Position<f32>, &mut Grounding, &mut Mass)),
        assertion_fn: fn((&Position<f32>, &Grounding)),
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(move |world| {
                let (mut positions, mut groundings, mut masses) = world.system_data::<(
                    WriteStorage<'_, Position<f32>>,
                    WriteStorage<'_, Grounding>,
                    WriteStorage<'_, Mass>,
                )>();
                (&mut positions, &mut groundings, &mut masses)
                    .join()
                    .for_each(setup_fn)
            })
            .with_system_single(
                ObjectGravitySystem::new(),
                ObjectGravitySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let (positions, groundings) = world
                    .system_data::<(ReadStorage<'_, Position<f32>>, ReadStorage<'_, Grounding>)>();
                (&positions, &groundings).join().for_each(assertion_fn)
            })
            .run_isolated()
    }
}

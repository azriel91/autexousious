use amethyst::{assets::AssetStorage, ecs::prelude::*};
use derive_new::new;
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use object_model::play::{Grounding, Position, Velocity};
use typename_derive::TypeName;

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterGroundingSystem;

type CharacterGroundingSystemData<'s> = (
    ReadExpect<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    WriteStorage<'s, Position<f32>>,
    WriteStorage<'s, Velocity<f32>>,
    WriteStorage<'s, Grounding>,
);

impl<'s> System<'s> for CharacterGroundingSystem {
    type SystemData = CharacterGroundingSystemData<'s>;

    fn run(
        &mut self,
        (map_selection, maps, mut positions, mut velocities, mut groundings): Self::SystemData,
    ) {
        let map_margins = {
            maps.get(map_selection.handle())
                .map(|map| map.margins)
                .expect("Expected map to be loaded.")
        };

        for (position, velocity, grounding) in
            (&mut positions, &mut velocities, &mut groundings).join()
        {
            // X axis
            if position[0] < map_margins.left {
                position[0] = map_margins.left;
            } else if position[0] > map_margins.right {
                position[0] = map_margins.right;
            }

            // Y axis
            if position[1] > map_margins.bottom {
                velocity[1] += -0.7;
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
    use object_model::play::{Grounding, Position};
    use typename::TypeName;

    use super::CharacterGroundingSystem;

    #[test]
    fn keeps_character_within_lower_map_bounds() -> Result<(), Error> {
        run_test(
            "keeps_character_within_lower_map_bounds",
            |(position, _grounding)| {
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
            "keeps_character_within_upper_map_bounds",
            |(position, _grounding)| {
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
            "grounding_set_to_airborne_when_above_ground",
            |(position, grounding)| {
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
            "grounding_set_to_on_ground_when_on_ground",
            |(position, grounding)| {
                position[1] = 200.;
                *grounding = Grounding::Airborne;
            },
            |(_position, grounding)| {
                assert_eq!(Grounding::OnGround, *grounding);
            },
        )
    }

    fn run_test(
        test_name: &str,
        setup_fn: fn((&mut Position<f32>, &mut Grounding)),
        assertion_fn: fn((&Position<f32>, &Grounding)),
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_app_name(test_name)
            .with_setup(move |world| {
                let (mut positions, mut groundings) = world
                    .system_data::<(WriteStorage<'_, Position<f32>>, WriteStorage<'_, Grounding>)>(
                    );
                (&mut positions, &mut groundings).join().for_each(setup_fn)
            })
            .with_system_single(
                CharacterGroundingSystem::new(),
                CharacterGroundingSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let (positions, groundings) = world
                    .system_data::<(ReadStorage<'_, Position<f32>>, ReadStorage<'_, Grounding>)>();
                (&positions, &groundings).join().for_each(assertion_fn)
            })
            .run()
    }
}

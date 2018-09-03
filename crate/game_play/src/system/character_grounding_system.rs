use amethyst::{assets::AssetStorage, ecs::prelude::*};
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::entity::{CharacterStatus, Grounding, Kinematics};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterGroundingSystem;

type CharacterGroundingSystemData<'s> = (
    Read<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, CharacterStatus>,
);

impl<'s> System<'s> for CharacterGroundingSystem {
    type SystemData = CharacterGroundingSystemData<'s>;

    fn run(
        &mut self,
        (map_selection, maps, mut kinematics_storage, mut status_storage): Self::SystemData,
    ) {
        let map_handle = map_selection
            .map_handle
            .as_ref()
            .expect("Expected map to be selected.");

        let map_margins = {
            maps.get(map_handle)
                .map(|map| map.margins)
                .expect("Expected map to be loaded.")
        };

        for (mut kinematics, mut status) in (&mut kinematics_storage, &mut status_storage).join() {
            // X axis
            if kinematics.position[0] < map_margins.left {
                kinematics.position[0] = map_margins.left;
            } else if kinematics.position[0] > map_margins.right {
                kinematics.position[0] = map_margins.right;
            }

            // Y axis
            if kinematics.position[1] > map_margins.bottom {
                kinematics.velocity[1] += -1.7;
                status.object_status.grounding = Grounding::Airborne;

                if kinematics.position[1] > map_margins.top {
                    kinematics.position[1] = map_margins.top;
                }
            } else {
                kinematics.position[1] = map_margins.bottom;
                kinematics.velocity[1] = 0.;
                status.object_status.grounding = Grounding::OnGround;
            }

            // Z axis
            if kinematics.position[2] < map_margins.back {
                kinematics.position[2] = map_margins.back;
            } else if kinematics.position[2] > map_margins.front {
                kinematics.position[2] = map_margins.front;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::ecs::prelude::*;
    use application_test_support::AutexousiousApplication;
    use object_model::entity::{CharacterStatus, Grounding, Kinematics};
    use typename::TypeName;

    use super::CharacterGroundingSystem;

    #[test]
    fn keeps_character_within_lower_map_bounds() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("keeps_character_within_lower_map_bounds", false)
                .with_setup(|world| {
                    world.exec(
                        |(mut kinematics_storage, status_storage): (
                            WriteStorage<Kinematics<f32>>,
                            ReadStorage<CharacterStatus>,
                        )| {
                            for (kinematics, _) in (&mut kinematics_storage, &status_storage).join()
                            {
                                kinematics.position[0] = -10.;
                                kinematics.position[1] = -10.;
                                kinematics.position[2] = -10.;
                            }
                        },
                    );
                }).with_system_single(
                    CharacterGroundingSystem::new(),
                    CharacterGroundingSystem::type_name(),
                    &[]
                ).with_assertion(|world| {
                    world.exec(
                        |(kinematics_storage, status_storage): (
                            ReadStorage<Kinematics<f32>>,
                            ReadStorage<CharacterStatus>,
                        )| {
                            for (kinematics, _) in (&kinematics_storage, &status_storage).join() {
                                assert_eq!(1., kinematics.position[0]);

                                // Map margins are shifted by z and depth. See
                                // `map_model::loaded::Margins`
                                assert_eq!(205., kinematics.position[1]);
                                assert_eq!(3., kinematics.position[2]);
                            }
                        },
                    );
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn keeps_character_within_upper_map_bounds() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("keeps_character_within_upper_map_bounds", false)
                .with_setup(|world| {
                    world.exec(
                        |(mut kinematics_storage, status_storage): (
                            WriteStorage<Kinematics<f32>>,
                            ReadStorage<CharacterStatus>,
                        )| {
                            for (kinematics, _) in (&mut kinematics_storage, &status_storage).join()
                            {
                                kinematics.position[0] = 2000.;
                                kinematics.position[1] = 2000.;
                                kinematics.position[2] = 2000.;
                            }
                        },
                    );
                }).with_system_single(
                    CharacterGroundingSystem::new(),
                    CharacterGroundingSystem::type_name(),
                    &[]
                ).with_assertion(|world| {
                    world.exec(
                        |(kinematics_storage, status_storage): (
                            ReadStorage<Kinematics<f32>>,
                            ReadStorage<CharacterStatus>,
                        )| {
                            for (kinematics, _) in (&kinematics_storage, &status_storage).join() {
                                assert_eq!(801., kinematics.position[0]);

                                // Map margins are shifted by z and depth. See
                                // `map_model::loaded::Margins`
                                assert_eq!(605., kinematics.position[1]);
                                assert_eq!(203., kinematics.position[2]);
                            }
                        },
                    );
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn grounding_set_to_airborne_when_above_ground() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base(
                "grounding_set_to_airborne_when_above_ground",
                false
            ).with_setup(|world| {
                world.exec(
                    |(mut kinematics_storage, mut status_storage): (
                        WriteStorage<Kinematics<f32>>,
                        WriteStorage<CharacterStatus>,
                    )| {
                        for (kinematics, status) in
                            (&mut kinematics_storage, &mut status_storage).join()
                        {
                            kinematics.position[1] = 300.;
                            status.object_status.grounding = Grounding::OnGround;
                        }
                    },
                );
            }).with_system_single(
                CharacterGroundingSystem::new(),
                CharacterGroundingSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                world.exec(
                    |(kinematics_storage, status_storage): (
                        ReadStorage<Kinematics<f32>>,
                        ReadStorage<CharacterStatus>,
                    )| {
                        for (_, status) in (&kinematics_storage, &status_storage).join() {
                            assert_eq!(Grounding::Airborne, status.object_status.grounding);
                        }
                    },
                );
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn grounding_set_to_on_ground_when_on_ground() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("grounding_set_to_on_ground_when_on_ground", false)
                .with_setup(|world| {
                    world.exec(
                        |(mut kinematics_storage, mut status_storage): (
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<CharacterStatus>,
                        )| {
                            for (kinematics, status) in
                                (&mut kinematics_storage, &mut status_storage).join()
                            {
                                kinematics.position[1] = 200.;
                                status.object_status.grounding = Grounding::Airborne;
                            }
                        },
                    );
                }).with_system_single(
                    CharacterGroundingSystem::new(),
                    CharacterGroundingSystem::type_name(),
                    &[]
                ).with_assertion(|world| {
                    world.exec(
                        |(kinematics_storage, status_storage): (
                            ReadStorage<Kinematics<f32>>,
                            ReadStorage<CharacterStatus>,
                        )| {
                            for (_, status) in (&kinematics_storage, &status_storage).join() {
                                assert_eq!(Grounding::OnGround, status.object_status.grounding);
                            }
                        },
                    );
                }).run()
                .is_ok()
        );
    }
}

#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Join, Read, ReadStorage, WriteStorage},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use game_input::ControllerInput;
    use kinematic_model::config::Position;
    use map_model::loaded::AssetMargins;
    use map_selection_model::MapSelection;
    use mirrored_model::play::Mirrored;
    use object_model::play::Grounding;
    use sequence_model::{loaded::SequenceId, play::SequenceStatus};

    use game_play::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_walk_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        controller_input.z_axis_value = -1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(3),
                mirrored: Mirrored(true),
            },
        )
    }

    #[test]
    fn flipped_is_none_when_walking_right() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(3),
                mirrored: Mirrored(false),
            },
        )
    }

    fn run_test(
        SetupParams {
            sequence_id: setup_sequence_id,
            controller_input: setup_controller_input,
            mirrored: setup_mirrored,
        }: SetupParams,
        ExpectedParams {
            sequence_id: expected_sequence_id,
            mirrored: expected_mirrored,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(move |world| {
                let (
                    map_selection,
                    asset_margins,
                    mut controller_inputs,
                    mut sequence_ids,
                    mut sequence_statuses,
                    mut positions,
                    mut mirroreds,
                    mut groundings,
                ) = world.system_data::<TestSystemData>();

                let margins = asset_margins
                    .get(
                        map_selection
                            .asset_id()
                            .expect("Expected `MapSelection` to have asset ID."),
                    )
                    .expect("Expected map to be loaded.");

                (
                    &mut controller_inputs,
                    &mut sequence_ids,
                    &mut sequence_statuses,
                    &mut positions,
                    &mut mirroreds,
                    &mut groundings,
                )
                    .join()
                    .for_each(
                        |(
                            controller_input,
                            sequence_id,
                            sequence_status,
                            position,
                            mirrored,
                            grounding,
                        )| {
                            *controller_input = setup_controller_input;

                            *sequence_id = setup_sequence_id;
                            *sequence_status = SequenceStatus::Ongoing;
                            *mirrored = setup_mirrored;
                            *grounding = Grounding::OnGround;

                            position[1] = margins.bottom;
                        },
                    );
            })
            .with_system_single(
                CharacterSequenceUpdateSystem::new(),
                any::type_name::<CharacterSequenceUpdateSystem>(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                world.exec(
                    |(sequence_ids, mirroreds): (
                        ReadStorage<'_, SequenceId>,
                        ReadStorage<'_, Mirrored>,
                    )| {
                        for (sequence_id, mirrored) in (&sequence_ids, &mirroreds).join() {
                            assert_eq!(expected_sequence_id, *sequence_id);
                            assert_eq!(expected_mirrored, *mirrored);
                        }
                    },
                );
            })
            .run_isolated()
    }

    type TestSystemData<'s> = (
        Read<'s, MapSelection>,
        Read<'s, AssetMargins>,
        WriteStorage<'s, ControllerInput>,
        WriteStorage<'s, SequenceId>,
        WriteStorage<'s, SequenceStatus>,
        WriteStorage<'s, Position<f32>>,
        WriteStorage<'s, Mirrored>,
        WriteStorage<'s, Grounding>,
    );

    #[derive(Debug)]
    struct SetupParams {
        sequence_id: SequenceId,
        controller_input: ControllerInput,
        mirrored: Mirrored,
    }

    #[derive(Debug)]
    struct ExpectedParams {
        sequence_id: SequenceId,
        mirrored: Mirrored,
    }
}

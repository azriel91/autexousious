use amethyst::{
    assets::AssetStorage,
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_model::{
    config::CharacterSequenceName,
    loaded::{Character, CharacterHandle},
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use kinematic_model::config::Velocity;
use object_model::play::Mirrored;
use sequence_model::{config::SequenceNameString, loaded::SequenceId, play::SequenceUpdateEvent};
use typename_derive::TypeName;

/// Updates `Character` velocity based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterKinematicsSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterKinematicsSystemData<'s> {
    /// `SequenceUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `CharacterHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_handles: ReadStorage<'s, CharacterHandle>,
    /// `Character` assets.
    #[derivative(Debug = "ignore")]
    pub character_assets: Read<'s, AssetStorage<Character>>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, SequenceId>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
}

impl<'s> System<'s> for CharacterKinematicsSystem {
    type SystemData = CharacterKinematicsSystemData<'s>;

    fn run(
        &mut self,
        CharacterKinematicsSystemData {
            sequence_update_ec,
            character_handles,
            character_assets,
            controller_inputs,
            sequence_ids,
            mut velocities,
            mut mirroreds,
        }: Self::SystemData,
    ) {
        for (character_handle, controller_input, sequence_id, velocity, mirrored) in (
            &character_handles,
            &controller_inputs,
            &sequence_ids,
            &mut velocities,
            &mut mirroreds,
        )
            .join()
        {
            // TODO: Character stats should be configuration.
            // Use a stats component from the character definition.

            // TODO: We shouldn't be relying on `CharacterHandle` to retrieve SequenceIdMappings`.
            let character = character_assets
                .get(character_handle)
                .expect("Expected `Character` to be loaded.");
            let character_sequence_name = character
                .sequence_id_mappings
                .name(*sequence_id)
                .expect("Expected sequence ID mapping to exist.");

            if let SequenceNameString::Name(character_sequence_name) = character_sequence_name {
                match character_sequence_name {
                    CharacterSequenceName::Stand
                    | CharacterSequenceName::Flinch0
                    | CharacterSequenceName::Flinch1 => {
                        velocity[0] = 0.;
                        velocity[2] = 0.;
                    }
                    CharacterSequenceName::Walk => {
                        velocity[0] = controller_input.x_axis_value as f32 * 3.5;
                        velocity[2] = controller_input.z_axis_value as f32 * 2.;
                    }
                    CharacterSequenceName::Run => {
                        velocity[0] = controller_input.x_axis_value as f32 * 6.;
                        velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                    }
                    CharacterSequenceName::RunStop => {
                        velocity[0] = if mirrored.0 { -2. } else { 2. };
                        velocity[2] = controller_input.z_axis_value as f32 * 0.5;
                    }
                    // TODO: velocity as config
                    CharacterSequenceName::Dodge => {
                        velocity[0] = if mirrored.0 { -3. } else { 3. };
                        velocity[2] = controller_input.z_axis_value as f32;
                    }
                    CharacterSequenceName::JumpDescendLand
                    | CharacterSequenceName::DashDescendLand
                    | CharacterSequenceName::FallForwardLand
                    | CharacterSequenceName::LieFaceDown
                    | CharacterSequenceName::StandAttack0
                    | CharacterSequenceName::StandAttack1 => {
                        velocity[0] /= 2.;
                        velocity[1] = 0.;
                        velocity[2] /= 2.;
                    }
                    _ => {}
                };
            }
        }

        sequence_update_ec
            .read(
                self.sequence_update_event_rid
                    .as_mut()
                    .expect("Expected `sequence_update_event_rid` to exist."),
            )
            .for_each(|ev| {
                if let SequenceUpdateEvent::SequenceBegin {
                    entity,
                    sequence_id,
                } = ev
                {
                    if let (
                        Some(character_handle),
                        Some(mirrored),
                        Some(controller_input),
                        Some(velocity),
                    ) = (
                        character_handles.get(*entity),
                        mirroreds.get(*entity),
                        controller_inputs.get(*entity),
                        velocities.get_mut(*entity),
                    ) {
                        let character = character_assets
                            .get(character_handle)
                            .expect("Expected `Character` to be loaded.");
                        let character_sequence_name = character
                            .sequence_id_mappings
                            .name(*sequence_id)
                            .expect("Expected sequence ID mapping to exist.");

                        if let SequenceNameString::Name(character_sequence_name) =
                            character_sequence_name
                        {
                            match character_sequence_name {
                                CharacterSequenceName::StandAttack0
                                | CharacterSequenceName::StandAttack1 => {
                                    velocity[0] = controller_input.x_axis_value as f32 * 2.5;
                                    velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                                }
                                CharacterSequenceName::JumpOff => {
                                    velocity[0] = controller_input.x_axis_value as f32 * 5.;
                                    velocity[1] = 10.5;
                                    velocity[2] = controller_input.z_axis_value as f32 * 2.;
                                }
                                CharacterSequenceName::DashForward => {
                                    velocity[0] = if mirrored.0 { -8. } else { 8. };
                                    velocity[1] = 7.5;
                                    velocity[2] = controller_input.z_axis_value as f32 * 2.5;
                                }
                                CharacterSequenceName::DashBack => {
                                    velocity[0] = if mirrored.0 { 11. } else { -11. };
                                    velocity[1] = 7.5;
                                    velocity[2] = controller_input.z_axis_value as f32 * 2.5;
                                }
                                _ => {}
                            };
                        }
                    }
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, Write, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_model::{
        config::CharacterSequenceName,
        loaded::{Character, CharacterHandle},
    };
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::play::{Grounding, Mirrored};
    use sequence_model::{
        config::SequenceNameString,
        loaded::{SequenceId, SequenceIdMappings},
        play::SequenceUpdateEvent,
    };
    use typename::TypeName;

    use super::CharacterKinematicsSystem;

    type EventFunction =
        fn(Entity, &SequenceIdMappings<CharacterSequenceName>) -> SequenceUpdateEvent;

    #[test]
    fn stand_x_and_z_velocity_are_zero() -> Result<(), Error> {
        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(3., 0., 3.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::Stand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(0., 0., 0.),
        )
    }

    #[test]
    fn updates_stand_attack_x_and_z_velocity() -> Result<(), Error> {
        vec![
            CharacterSequenceName::StandAttack0,
            CharacterSequenceName::StandAttack1,
        ]
        .into_iter()
        .try_for_each(|stand_attack_id| {
            let mut controller_input = ControllerInput::default();
            controller_input.x_axis_value = 1.;
            controller_input.z_axis_value = -1.;

            run_test(
                SetupParams {
                    velocity: Velocity::new(0., 0., 0.),
                    grounding: Grounding::OnGround,
                    character_sequence_name: stand_attack_id.clone(),
                    controller_input: Some(controller_input),
                    mirrored: None,
                    event_fn:
                        Some(
                            move |entity,
                                  sequence_id_mappings: &SequenceIdMappings<
                                CharacterSequenceName,
                            >| {
                                let sequence_id = *sequence_id_mappings
                                    .id(&SequenceNameString::Name(stand_attack_id))
                                    .expect("Expected mapping for sequence ID to exist.");
                                SequenceUpdateEvent::SequenceBegin {
                                    entity,
                                    sequence_id,
                                }
                            },
                        ),
                },
                Velocity::new(2.5, 0., -1.5),
            )
        })
    }

    #[test]
    fn updates_walk_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.z_axis_value = -1.;

        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::Walk,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(3.5, 0., -2.),
        )
    }

    #[test]
    fn updates_run_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.z_axis_value = -1.;

        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::Run,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(6., 0., -1.5),
        )
    }

    #[test]
    fn updates_run_stop_x_and_z_velocity() -> Result<(), Error> {
        vec![(false, 2.), (true, -2.)].into_iter().try_for_each(
            |(mirrored_bool, vx)| -> Result<(), Error> {
                let mut controller_input = ControllerInput::default();
                controller_input.z_axis_value = 1.;

                run_test::<EventFunction>(
                    SetupParams {
                        velocity: Velocity::new(0., 0., 0.),
                        grounding: Grounding::OnGround,
                        character_sequence_name: CharacterSequenceName::RunStop,
                        controller_input: Some(controller_input),
                        mirrored: Some(mirrored_bool.into()),
                        event_fn: None,
                    },
                    Velocity::new(vx, 0., 0.5),
                )
            },
        )
    }

    #[test]
    fn updates_dodge_x_and_z_velocity() -> Result<(), Error> {
        vec![(false, 3.), (true, -3.)].into_iter().try_for_each(
            |(mirrored_bool, vx)| -> Result<(), Error> {
                let mut controller_input = ControllerInput::default();
                controller_input.z_axis_value = 1.;

                run_test::<EventFunction>(
                    SetupParams {
                        velocity: Velocity::new(0., 0., 0.),
                        grounding: Grounding::OnGround,
                        character_sequence_name: CharacterSequenceName::Dodge,
                        controller_input: Some(controller_input),
                        mirrored: Some(mirrored_bool.into()),
                        event_fn: None,
                    },
                    Velocity::new(vx, 0., 1.),
                )
            },
        )
    }

    #[test]
    fn updates_jump_off_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::JumpOff,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(
                    |entity, sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>| {
                        let sequence_id = *sequence_id_mappings
                            .id(&SequenceNameString::Name(CharacterSequenceName::JumpOff))
                            .expect("Expected mapping for sequence ID to exist.");
                        SequenceUpdateEvent::SequenceBegin {
                            entity,
                            sequence_id,
                        }
                    },
                ),
            },
            Velocity::new(-5., 10.5, 2.),
        )
    }

    #[test]
    fn updates_dash_forward_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::DashForward,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(
                    |entity, sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>| {
                        let sequence_id = *sequence_id_mappings
                            .id(&SequenceNameString::Name(
                                CharacterSequenceName::DashForward,
                            ))
                            .expect("Expected mapping for sequence ID to exist.");
                        SequenceUpdateEvent::SequenceBegin {
                            entity,
                            sequence_id,
                        }
                    },
                ),
            },
            Velocity::new(8., 7.5, 2.5),
        )
    }

    #[test]
    fn updates_dash_back_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::DashBack,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(
                    |entity, sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>| {
                        let sequence_id = *sequence_id_mappings
                            .id(&SequenceNameString::Name(CharacterSequenceName::DashBack))
                            .expect("Expected mapping for sequence ID to exist.");
                        SequenceUpdateEvent::SequenceBegin {
                            entity,
                            sequence_id,
                        }
                    },
                ),
            },
            Velocity::new(-11., 7.5, 2.5),
        )
    }

    #[test]
    fn updates_jump_descend_land_xyz_velocity() -> Result<(), Error> {
        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::JumpDescendLand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    #[test]
    fn updates_fall_forward_land_xyz_velocity() -> Result<(), Error> {
        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::FallForwardLand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    #[test]
    fn updates_lie_face_down_xyz_velocity() -> Result<(), Error> {
        run_test::<EventFunction>(
            SetupParams {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                character_sequence_name: CharacterSequenceName::LieFaceDown,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    fn run_test<FnEvt>(
        SetupParams {
            velocity: velocity_setup,
            grounding: grounding_setup,
            character_sequence_name: character_sequence_name_setup,
            controller_input: controller_input_setup,
            mirrored: mirrored_setup,
            event_fn,
            ..
        }: SetupParams<FnEvt>,
        velocity_expected: Velocity<f32>,
    ) -> Result<(), Error>
    where
        FnEvt: Fn(Entity, &SequenceIdMappings<CharacterSequenceName>) -> SequenceUpdateEvent
            + Send
            + Sync
            + 'static,
    {
        AutexousiousApplication::game_base()
            // kcov-ignore-start
            .with_system(
                CharacterKinematicsSystem::new(),
                CharacterKinematicsSystem::type_name(),
                &[],
            )
            // kcov-ignore-end
            .with_setup(move |world| {
                let (
                    entities,
                    map_selection,
                    maps,
                    character_handles,
                    character_assets,
                    mut sequence_ids,
                    mut positions,
                    mut velocities,
                    mut groundings,
                    mut controller_inputs,
                    mut mirroreds,
                    mut event_channel,
                ) = world.system_data::<TestSystemData>();
                let map = maps
                    .get(map_selection.handle())
                    .expect("Expected map to be loaded.");

                for (
                    entity,
                    character_handle,
                    sequence_id,
                    position,
                    velocity,
                    grounding,
                    controller_input,
                    mirrored,
                ) in (
                    &entities,
                    &character_handles,
                    &mut sequence_ids,
                    &mut positions,
                    &mut velocities,
                    &mut groundings,
                    &mut controller_inputs,
                    &mut mirroreds,
                )
                    .join()
                {
                    let character = character_assets
                        .get(character_handle)
                        .expect("Expected `Character` to be loaded.");
                    let sequence_id_mappings = &character.sequence_id_mappings;
                    let sequence_id_setup = sequence_id_mappings
                        .id(&SequenceNameString::Name(character_sequence_name_setup))
                        .expect("Expected sequence ID mapping to exist.");
                    *sequence_id = *sequence_id_setup;
                    *grounding = grounding_setup;

                    position[1] = map.margins.bottom;
                    *velocity = velocity_setup;

                    if let Some(controller_input_setup) = controller_input_setup {
                        *controller_input = controller_input_setup;
                    }
                    if let Some(mirrored_setup) = mirrored_setup {
                        *mirrored = mirrored_setup;
                    }
                    if let Some(event_fn) = &event_fn {
                        event_channel.single_write(event_fn(entity, sequence_id_mappings));
                    }
                }
            })
            .with_assertion(move |world| {
                world.exec(
                    |(sequence_ids, velocities): (
                        ReadStorage<'_, SequenceId>,
                        ReadStorage<'_, Velocity<f32>>,
                    )| {
                        for (_, velocity_actual) in (&sequence_ids, &velocities).join() {
                            assert_eq!(&velocity_expected, velocity_actual);
                        }
                    },
                );
            })
            .run_isolated()
    }

    #[derive(Debug)]
    struct SetupParams<FnEvt>
    where
        FnEvt: Fn(Entity, &SequenceIdMappings<CharacterSequenceName>) -> SequenceUpdateEvent
            + Send
            + Sync
            + 'static,
    {
        velocity: Velocity<f32>,
        character_sequence_name: CharacterSequenceName,
        grounding: Grounding,
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
        event_fn: Option<FnEvt>,
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        ReadExpect<'s, MapSelection>,
        Read<'s, AssetStorage<Map>>,
        ReadStorage<'s, CharacterHandle>,
        Read<'s, AssetStorage<Character>>,
        WriteStorage<'s, SequenceId>,
        WriteStorage<'s, Position<f32>>,
        WriteStorage<'s, Velocity<f32>>,
        WriteStorage<'s, Grounding>,
        WriteStorage<'s, ControllerInput>,
        WriteStorage<'s, Mirrored>,
        Write<'s, EventChannel<SequenceUpdateEvent>>,
    );
}

#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt, WriteStorage},
        input::{Button, InputEvent},
        shred::{ResourceId, SystemData},
        shrev::EventChannel,
        winit::VirtualKeyCode,
        Error,
    };
    use application::IoUtils;
    use application_test_support::AutexousiousApplication;
    use derivative::Derivative;
    use game_input_model::{play::ButtonInputControlled, ControlBindings};
    use input_reaction_loading::{IrsLoader, IrsLoaderParams};
    use input_reaction_model::{
        config::BasicIrr,
        loaded::{
            InputReaction, InputReactions, InputReactionsHandle, InputReactionsSequence,
            InputReactionsSequenceHandle,
        },
    };
    use sequence_model::{
        config::SequenceNameString,
        loaded::{SequenceId, SequenceIdMappings},
    };
    use sprite_model::config::SpriteSequenceName;
    use ui_model::config::UiSequence;

    use input_reaction_play::ButtonInputReactionsTransitionSystem;

    #[test]
    fn inserts_transition_for_button_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                input_event: Some(InputEvent::ButtonPressed(Button::Key(VirtualKeyCode::A))),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(1),
            },
        )
    }

    #[test]
    fn ignores_irrelevant_button_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                input_event: Some(InputEvent::ButtonPressed(Button::Key(VirtualKeyCode::B))),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(0),
            },
        )
    }

    fn run_test(
        SetupParams {
            sequence_id: sequence_id_setup,
            input_event,
        }: SetupParams,
        ExpectedParams {
            sequence_id: sequence_id_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                ButtonInputReactionsTransitionSystem::<BasicIrr>::new(),
                "",
                &[],
            )
            .with_effect(move |world| {
                let irs_handle = {
                    let (loader, input_reactions_assets, input_reactions_sequence_assets) = world
                        .system_data::<(
                            ReadExpect<'_, Loader>,
                            Read<'_, AssetStorage<InputReactions<InputReaction<BasicIrr>>>>,
                            Read<'_, AssetStorage<InputReactionsSequence<InputReaction<BasicIrr>>>>,
                        )>();

                    let irs_loader_params = IrsLoaderParams {
                        loader: &loader,
                        input_reactions_assets: &input_reactions_assets,
                        input_reactions_sequence_assets: &input_reactions_sequence_assets,
                    };
                    let button_sequence = button_sequence();

                    IrsLoader::load(
                        &irs_loader_params,
                        &sequence_id_mappings(),
                        None,
                        &button_sequence,
                    )
                };

                world.insert(irs_handle);
            })
            // Allow `AssetStorage`s to process loaded data.
            .with_effect(move |world| {
                let input_reactions_handle = {
                    let input_reactions_sequence_assets =
                        world.system_data::<Read<'_, AssetStorage<InputReactionsSequence>>>();

                    let irs_handle = world
                        .read_resource::<InputReactionsSequenceHandle<InputReaction<BasicIrr>>>()
                        .clone();
                    let irs = input_reactions_sequence_assets
                        .get(&irs_handle)
                        .expect("Expected `irs` to be loaded.");
                    irs.first()
                        .expect(
                            "Expected `irs` to contain one \
                             `input_reactions_handle`.",
                        )
                        .clone()
                };

                let entity = world
                    .create_entity()
                    .with(sequence_id_setup)
                    .with(input_reactions_handle)
                    .with(ButtonInputControlled)
                    .build();

                if let Some(input_event) = input_event {
                    send_event(world, input_event);
                }

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();

                let sequence_id = {
                    let sequence_ids = world.read_storage::<SequenceId>();

                    sequence_ids
                        .get(entity)
                        .copied()
                        .expect("Expected `SequenceId` component to exist.")
                };

                assert_eq!(sequence_id_expected, sequence_id);
            })
            .run_isolated()
    }

    fn button_sequence() -> UiSequence {
        let button_sequence_yaml = "button_input_reactions_transition_system_button_sequence.yaml";
        let button_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "input_reaction_play",
            "system",
            button_sequence_yaml,
        ]);
        let contents = IoUtils::read_file(&button_sequence_path)
            .unwrap_or_else(|e| panic!("Failed to read `{}`. Error: {}", button_sequence_yaml, e));

        serde_yaml::from_slice::<UiSequence>(&contents).expect(
            "Failed to load `button_input_reactions_transition_system_button_sequence.yaml`.",
        )
    }

    fn sequence_id_mappings() -> SequenceIdMappings<SpriteSequenceName> {
        let mut sequence_id_mappings = SequenceIdMappings::new();
        sequence_id_mappings.insert(
            SequenceNameString::String(String::from("zero")),
            SequenceId::new(0),
        );
        sequence_id_mappings.insert(
            SequenceNameString::String(String::from("one")),
            SequenceId::new(1),
        );
        sequence_id_mappings
    }

    fn send_event(world: &mut World, event: InputEvent<ControlBindings>) {
        let mut ec = world.write_resource::<EventChannel<InputEvent<ControlBindings>>>();
        ec.single_write(event);
    } // kcov-ignore

    #[derive(Derivative, SystemData)]
    #[derivative(Debug)]
    struct TestSystemData<'s> {
        #[derivative(Debug = "ignore")]
        sequence_ids: WriteStorage<'s, SequenceId>,
        #[derivative(Debug = "ignore")]
        input_reactions_handles: WriteStorage<'s, InputReactionsHandle<InputReaction<BasicIrr>>>,
        #[derivative(Debug = "ignore")]
        button_input_controlleds: WriteStorage<'s, ButtonInputControlled>,
    }

    struct SetupParams {
        sequence_id: SequenceId,
        input_event: Option<InputEvent<ControlBindings>>,
    }

    struct ExpectedParams {
        sequence_id: SequenceId,
    }
}

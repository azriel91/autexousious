use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader},
    core::TransformBundle,
    ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
    renderer::{
        loaders::load_from_srgba,
        palette::Srgba,
        sprite::{Sprite, SpriteSheet, SpriteSheetHandle},
        types::{DefaultBackend, TextureData},
        RenderEmptyBundle, Texture,
    },
    Error,
};
use amethyst_test::AmethystApplication;
use character_loading::{CharacterLoadingBundle, CHARACTER_PROCESSOR};
use character_model::{
    config::{
        CharacterDefinition, CharacterSequence, CharacterSequenceName, ControlTransitionRequirement,
    },
    loaded::{
        Character, CharacterControlTransition, CharacterControlTransitions,
        CharacterControlTransitionsSequence, CharacterHandle,
    },
};
use character_prefab::{CharacterPrefab, CharacterPrefabBundle, CharacterPrefabHandle};
use charge_model::config::{
    ChargeDelay, ChargeLimit, ChargePoints, ChargeRetentionMode, ChargeUseMode,
};
use game_input_model::ControlAction;
use indexmap::IndexMap;
use object_model::{
    config::{ObjectAssetData, ObjectDefinition, ObjectFrame, ObjectSequence},
    play::{HealthPoints, SkillPoints},
};
use pretty_assertions::assert_eq;
use sequence_loading::SequenceLoadingBundle;
use sequence_model::{
    config::{SequenceEndTransition, SequenceNameString},
    loaded::{
        ActionHold, ActionPress, ActionRelease, ControlTransition, ControlTransitions, SequenceId,
    },
};

#[test]
fn character_prefab_load() -> Result<(), Error> {
    AmethystApplication::blank()
        .with_bundle(TransformBundle::new())
        .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
        .with_bundle(SequenceLoadingBundle::new())
        .with_bundle(CharacterLoadingBundle::new())
        .with_bundle(
            CharacterPrefabBundle::new()
                .with_system_dependencies(&[String::from(CHARACTER_PROCESSOR)]),
        )
        .with_setup(|world| {
            let character_prefab_handle = {
                let (loader, character_definition_assets, character_prefab_loader) =
                    world.system_data::<TestSystemData>();
                let character_definition_handle =
                    loader.load_from_data(character_definition(), (), &character_definition_assets);
                let object_asset_data =
                    ObjectAssetData::new(character_definition_handle, sprite_sheet_handles(&world));
                let character_prefab = CharacterPrefab::new(object_asset_data);
                character_prefab_loader.load_from_data(Prefab::new_main(character_prefab), ())
            };
            world.insert(character_prefab_handle);
        })
        .with_setup(|_world| {}) // Allow texture to load.
        .with_setup(|world| {
            let character_prefab_handle = (*world.read_resource::<CharacterPrefabHandle>()).clone();
            let character_entity = world.create_entity().with(character_prefab_handle).build();
            world.insert(character_entity);
        })
        .with_effect(|_world| {})
        .with_assertion(|world| {
            let character_entity = *world.read_resource::<Entity>();
            let character_handles = world.read_storage::<CharacterHandle>();
            let character_handle = character_handles
                .get(character_entity)
                .expect("Expected entity to have `CharacterHandle` component.");
            let character_assets = world.read_resource::<AssetStorage<Character>>();
            let character = character_assets
                .get(character_handle)
                .expect("Expected `Character` to be loaded.");
            let character_control_transitions_sequence_assets =
                world.read_resource::<AssetStorage<CharacterControlTransitionsSequence>>();
            let character_control_transitions_sequences = {
                let handle = character
                    .control_transitions_sequence_handles
                    .get(*SequenceId::new(0))
                    .expect("Expected `CharacterControlTransitionsSequenceHandle` to exist.");

                character_control_transitions_sequence_assets
                    .get(handle)
                    .expect("Expected `CharacterControlTransitionsSequence` to be loaded.")
            };

            // Assert the values for each handle.
            let character_control_transitions_assets =
                world.read_resource::<AssetStorage<CharacterControlTransitions>>();

            let expected_character_control_transitions = expected_control_transitions();
            let character_control_transitions_handle = character_control_transitions_sequences
                .first()
                .expect("Expected `CharacterControlTransitionsHandle` to exist.");
            let character_control_transitions = character_control_transitions_assets
                .get(character_control_transitions_handle)
                .expect("Expected `CharacterControlTransitions` to be loaded.");
            assert_eq!(
                &expected_character_control_transitions,
                character_control_transitions
            );
        })
        .run_isolated()
}

fn character_definition() -> CharacterDefinition {
    use character_model::config::{CharacterControlTransitions, CharacterFrame};
    use sequence_model::config::{
        ControlTransition, ControlTransitionMultiple, ControlTransitionSingle, Wait,
    };

    let frames = vec![CharacterFrame::new(
        ObjectFrame {
            wait: Wait::new(5),
            ..Default::default()
        },
        CharacterControlTransitions {
            press_attack: Some(ControlTransition::SequenceNameString(
                SequenceNameString::Name(CharacterSequenceName::StandAttack0),
            )),
            release_attack: Some(ControlTransition::Multiple(ControlTransitionMultiple::new(
                vec![
                    ControlTransitionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::Walk),
                        requirements: vec![ControlTransitionRequirement::Charge(
                            ChargePoints::new(90),
                        )],
                    },
                    ControlTransitionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::Run),
                        requirements: vec![ControlTransitionRequirement::Sp(SkillPoints::new(50))],
                    },
                    ControlTransitionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::RunStop),
                        requirements: vec![ControlTransitionRequirement::Hp(HealthPoints::new(30))],
                    },
                ],
            ))),
            hold_jump: Some(ControlTransition::Single(ControlTransitionSingle {
                next: SequenceNameString::Name(CharacterSequenceName::Jump),
                requirements: vec![],
            })),
            ..Default::default()
        }, // kcov-ignore
    )];
    let sequence = CharacterSequence::new(
        ObjectSequence::new(
            SequenceEndTransition::SequenceName(SequenceNameString::Name(
                CharacterSequenceName::Stand,
            )),
            frames,
        ),
        None,
    );
    let mut sequences = IndexMap::new();
    // 0
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::Stand),
        sequence,
    );
    // 1
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::StandAttack0),
        empty_sequence(),
    );
    // 2
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::Walk),
        empty_sequence(),
    );
    // 3
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::Run),
        empty_sequence(),
    );
    // 4
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::RunStop),
        empty_sequence(),
    );
    // 5
    sequences.insert(
        SequenceNameString::Name(CharacterSequenceName::Jump),
        empty_sequence(),
    );
    let object_definition = ObjectDefinition::new(sequences);

    CharacterDefinition {
        object_definition,
        charge_limit: ChargeLimit::new(50),
        charge_delay: ChargeDelay::new(20),
        charge_use_mode: ChargeUseMode::Exact,
        charge_retention_mode: ChargeRetentionMode::Lossy { delay: 5 },
    }
}

fn empty_sequence() -> CharacterSequence {
    let object_sequence = ObjectSequence::new(Default::default(), Vec::new());
    CharacterSequence::new(object_sequence, None)
}

fn sprite_sheet_handles(world: &World) -> Vec<SpriteSheetHandle> {
    let loader = world.read_resource::<Loader>();
    let texture_assets = world.read_resource::<AssetStorage<Texture>>();
    let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 0.));
    let texture_handle: Handle<Texture> =
        loader.load_from_data(TextureData::from(texture_builder), (), &texture_assets);

    let image_w = 1;
    let image_h = 1;
    let sprite_w = 1;
    let sprite_h = 1;
    let pixel_left = 0;
    let pixel_top = 0;
    let offsets = [0.; 2];

    let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();
    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites: vec![Sprite::from_pixel_values(
            image_w, image_h, sprite_w, sprite_h, pixel_left, pixel_top, offsets, false, false,
        )],
    }; // kcov-ignore

    vec![loader.load_from_data(sprite_sheet, (), &sprite_sheet_assets)]
}

type TestSystemData<'s> = (
    ReadExpect<'s, Loader>,
    Read<'s, AssetStorage<CharacterDefinition>>,
    PrefabLoader<'s, CharacterPrefab>,
);

fn expected_control_transitions() -> CharacterControlTransitions {
    CharacterControlTransitions::new(ControlTransitions::new(vec![
        CharacterControlTransition {
            control_transition: ControlTransition::ActionPress(ActionPress {
                action: ControlAction::Jump,
                sequence_id: SequenceId::new(5),
            }),
            control_transition_requirements: vec![],
        },
        CharacterControlTransition {
            control_transition: ControlTransition::ActionPress(ActionPress {
                action: ControlAction::Attack,
                sequence_id: SequenceId::new(1),
            }),
            control_transition_requirements: vec![],
        },
        CharacterControlTransition {
            control_transition: ControlTransition::ActionRelease(ActionRelease {
                action: ControlAction::Attack,
                sequence_id: SequenceId::new(2),
            }),
            control_transition_requirements: vec![ControlTransitionRequirement::Charge(
                ChargePoints::new(90),
            )],
        },
        CharacterControlTransition {
            control_transition: ControlTransition::ActionRelease(ActionRelease {
                action: ControlAction::Attack,
                sequence_id: SequenceId::new(3),
            }),
            control_transition_requirements: vec![ControlTransitionRequirement::Sp(
                SkillPoints::new(50),
            )],
        },
        CharacterControlTransition {
            control_transition: ControlTransition::ActionRelease(ActionRelease {
                action: ControlAction::Attack,
                sequence_id: SequenceId::new(4),
            }),
            control_transition_requirements: vec![ControlTransitionRequirement::Hp(
                HealthPoints::new(30),
            )],
        },
        CharacterControlTransition {
            control_transition: ControlTransition::ActionHold(ActionHold {
                action: ControlAction::Jump,
                sequence_id: SequenceId::new(5),
            }),
            control_transition_requirements: vec![],
        },
    ]))
}

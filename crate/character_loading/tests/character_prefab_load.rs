use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader, Prefab, PrefabLoader},
    ecs::{Builder, Entity, Read, ReadExpect, World},
    renderer::{Sprite, SpriteSheet, SpriteSheetHandle, Texture, TextureHandle},
    Error,
};
use amethyst_test::AmethystApplication;
use character_loading::{CharacterLoadingBundle, CharacterPrefab, CharacterPrefabHandle};
use character_model::{
    config::{CharacterDefinition, CharacterSequenceId, ControlTransitionRequirement},
    loaded::{
        Character, CharacterControlTransition, CharacterControlTransitions,
        CharacterControlTransitionsSequence, CharacterHandle,
    },
};
use collision_model::config::{Body, Interactions};
use game_input_model::ControlAction;
use object_model::{
    config::{ObjectAssetData, ObjectDefinition, ObjectFrame, ObjectSequence},
    play::{ChargePoints, HealthPoints, SkillPoints},
};
use pretty_assertions::assert_eq;
use sequence_loading::SequenceLoadingBundle;
use sequence_model::loaded::{
    ControlTransition, ControlTransitionHold, ControlTransitionPress, ControlTransitionRelease,
    ControlTransitions,
};
use sprite_model::config::SpriteRef;

#[test]
fn character_prefab_load() -> Result<(), Error> {
    AmethystApplication::render_base("character_prefab_load", false)
        .with_bundle(SequenceLoadingBundle::new())
        .with_bundle(CharacterLoadingBundle::new())
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
            world.add_resource(character_prefab_handle);
        })
        .with_setup(|_world| {}) // Allow texture to load.
        .with_setup(|world| {
            let character_prefab_handle = world.read_resource::<CharacterPrefabHandle>().clone();
            let character_entity = world.create_entity().with(character_prefab_handle).build();
            world.add_resource(character_entity);
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
                    .get(&CharacterSequenceId::Stand)
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
        .run()
}

fn character_definition() -> CharacterDefinition {
    use character_model::config::{CharacterControlTransitions, CharacterFrame, CharacterSequence};
    use sequence_model::config::{
        ControlTransition, ControlTransitionMultiple, ControlTransitionSingle, Wait,
    };

    let frames = vec![CharacterFrame::new(
        ObjectFrame::new(
            Wait::new(5),
            SpriteRef::new(0, 0),
            Body::default(),
            Interactions::default(),
        ),
        CharacterControlTransitions {
            press_attack: Some(ControlTransition::SequenceId(
                CharacterSequenceId::StandAttack0,
            )),
            release_attack: Some(ControlTransition::Multiple(ControlTransitionMultiple::new(
                vec![
                    ControlTransitionSingle {
                        next: CharacterSequenceId::Walk,
                        extra: ControlTransitionRequirement {
                            charge: ChargePoints::new(90),
                            ..Default::default()
                        },
                    },
                    ControlTransitionSingle {
                        next: CharacterSequenceId::Run,
                        extra: ControlTransitionRequirement {
                            sp: SkillPoints::new(50),
                            ..Default::default()
                        },
                    },
                    ControlTransitionSingle {
                        next: CharacterSequenceId::RunStop,
                        extra: ControlTransitionRequirement {
                            hp: HealthPoints::new(30),
                            ..Default::default()
                        },
                    },
                ],
            ))),
            hold_jump: Some(ControlTransition::Single(ControlTransitionSingle {
                next: CharacterSequenceId::Jump,
                extra: ControlTransitionRequirement::default(),
            })),
            ..Default::default()
        }, // kcov-ignore
    )];
    let sequence = CharacterSequence::new(ObjectSequence::new(
        Some(CharacterSequenceId::Stand),
        frames,
    ));
    let mut sequences = HashMap::new();
    sequences.insert(CharacterSequenceId::Stand, sequence);
    let object_definition = ObjectDefinition::new(sequences);

    CharacterDefinition::new(object_definition)
}

fn sprite_sheet_handles(world: &World) -> Vec<SpriteSheetHandle> {
    let loader = world.read_resource::<Loader>();
    let texture_assets = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle: TextureHandle = loader.load_from_data([0.; 4].into(), (), &texture_assets);

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
            image_w, image_h, sprite_w, sprite_h, pixel_left, pixel_top, offsets,
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
            control_transition: ControlTransition::Press(ControlTransitionPress {
                action: ControlAction::Attack,
                sequence_id: CharacterSequenceId::StandAttack0,
            }),
            control_transition_requirement: None,
        },
        CharacterControlTransition {
            control_transition: ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Attack,
                sequence_id: CharacterSequenceId::Walk,
            }),
            control_transition_requirement: Some(ControlTransitionRequirement {
                charge: ChargePoints::new(90),
                ..Default::default()
            }),
        },
        CharacterControlTransition {
            control_transition: ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Attack,
                sequence_id: CharacterSequenceId::Run,
            }),
            control_transition_requirement: Some(ControlTransitionRequirement {
                sp: SkillPoints::new(50),
                ..Default::default()
            }),
        },
        CharacterControlTransition {
            control_transition: ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Attack,
                sequence_id: CharacterSequenceId::RunStop,
            }),
            control_transition_requirement: Some(ControlTransitionRequirement {
                hp: HealthPoints::new(30),
                ..Default::default()
            }),
        },
        CharacterControlTransition {
            control_transition: ControlTransition::Hold(ControlTransitionHold {
                action: ControlAction::Jump,
                sequence_id: CharacterSequenceId::Jump,
            }),
            control_transition_requirement: None,
        },
    ]))
}

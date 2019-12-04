mod character_sequence;
mod input_reaction_requirement;

#[cfg(test)]
mod test {
    use charge_model::config::ChargePoints;
    use collision_model::config::Body;
    use indexmap::IndexMap;
    use input_reaction_model::config::{
        InputReaction, InputReactionAppEvents, InputReactionMultiple, InputReactionSingle,
    };
    use object_model::{
        config::{ObjectDefinition, ObjectFrame, ObjectSequence},
        play::{HealthPoints, SkillPoints},
    };
    use sequence_model::config::{SequenceEndTransition, SequenceNameString, Wait};
    use serde_yaml;
    use shape_model::Volume;
    use sprite_model::config::SpriteRef;

    use character_model::config::{
        CharacterDefinition, CharacterFrame, CharacterInputReactions, CharacterSequence,
        CharacterSequenceName, InputReactionRequirement,
    };

    const OBJECT_YAML: &str = "\
sequences:
  stand:
    next: 'walk'
    input_reactions: { press_defend: 'stand_attack_1' }
    frames:
      - wait: 5
        sprite: { sheet: 1, index: 3 }
        body: [{ box: { x: 25, y: 11, w: 31, h: 68 } }]
        input_reactions:
          press_attack: 'stand_attack_0'
          release_attack:
            - { next: 'walk', requirements: [{ charge: 90 }] }
            - { next: 'run', requirements: [{ sp: 50 }] }
            - { next: 'run_stop', requirements: [{ hp: 30 }] }
          hold_jump: { next: 'jump' }

  custom_sequence_0:
    next: 'custom_sequence_1'
    input_reactions: { press_defend: 'custom_sequence_4' }
    frames:
      - wait: 5
        sprite: { sheet: 1, index: 3 }
        input_reactions:
          press_attack: 'custom_sequence_1'
          release_attack:
            - { next: 'custom_sequence_2' }
          hold_jump: { next: 'custom_sequence_3' }

  custom_sequence_1:
    next: 'stand'
    frames: []
  custom_sequence_2:
    next: 'stand'
    frames: []
  custom_sequence_3:
    next: 'stand'
    frames: []
  custom_sequence_4:
    next: 'stand'
    frames: []
";

    #[test]
    fn deserialize_character_definition() {
        let char_definition = serde_yaml::from_str::<CharacterDefinition>(OBJECT_YAML)
            .expect("Failed to deserialize character definition.");

        let mut sequences = IndexMap::new();
        sequences.insert(
            SequenceNameString::Name(CharacterSequenceName::Stand),
            stand_sequence(),
        );
        sequences.insert(
            SequenceNameString::String(String::from("custom_sequence_0")),
            custom_sequence_0(),
        );
        sequences.insert(
            SequenceNameString::String(String::from("custom_sequence_1")),
            empty_sequence(),
        );
        sequences.insert(
            SequenceNameString::String(String::from("custom_sequence_2")),
            empty_sequence(),
        );
        sequences.insert(
            SequenceNameString::String(String::from("custom_sequence_3")),
            empty_sequence(),
        );
        sequences.insert(
            SequenceNameString::String(String::from("custom_sequence_4")),
            empty_sequence(),
        );
        let object_definition = ObjectDefinition::new(sequences);
        let expected = CharacterDefinition {
            object_definition,
            ..Default::default()
        };
        assert_eq!(expected, char_definition);
    }

    fn stand_sequence() -> CharacterSequence {
        let frames = vec![CharacterFrame::new(
            ObjectFrame {
                wait: Wait::new(5),
                sprite: SpriteRef::new(1, 3),
                body: Body::new(vec![Volume::Box {
                    x: 25,
                    y: 11,
                    z: 0,
                    w: 31,
                    h: 68,
                    d: 26,
                }]),
                ..Default::default()
            },
            CharacterInputReactions {
                press_attack: Some(InputReaction::SequenceNameString(SequenceNameString::Name(
                    CharacterSequenceName::StandAttack0,
                ))),
                release_attack: Some(InputReaction::Multiple(InputReactionMultiple::new(vec![
                    InputReactionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::Walk),
                        events: InputReactionAppEvents::default(),
                        requirements: vec![InputReactionRequirement::Charge(ChargePoints::new(90))],
                    },
                    InputReactionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::Run),
                        events: InputReactionAppEvents::default(),
                        requirements: vec![InputReactionRequirement::Sp(SkillPoints::new(50))],
                    },
                    InputReactionSingle {
                        next: SequenceNameString::Name(CharacterSequenceName::RunStop),
                        events: InputReactionAppEvents::default(),
                        requirements: vec![InputReactionRequirement::Hp(HealthPoints::new(30))],
                    },
                ]))),
                hold_jump: Some(InputReaction::Single(InputReactionSingle {
                    next: SequenceNameString::Name(CharacterSequenceName::Jump),
                    events: InputReactionAppEvents::default(),
                    requirements: vec![],
                })),
                ..Default::default()
            }, // kcov-ignore
        )];

        let character_input_reactions = CharacterInputReactions {
            press_defend: Some(InputReaction::SequenceNameString(SequenceNameString::Name(
                CharacterSequenceName::StandAttack1,
            ))),
            ..Default::default()
        };
        CharacterSequence::new(
            ObjectSequence {
                next: SequenceEndTransition::SequenceName(SequenceNameString::Name(
                    CharacterSequenceName::Walk,
                )),
                frames,
                ..Default::default()
            },
            Some(character_input_reactions),
        )
    }

    fn custom_sequence_0() -> CharacterSequence {
        let frames = vec![CharacterFrame::new(
            ObjectFrame {
                wait: Wait::new(5),
                sprite: SpriteRef::new(1, 3),
                ..Default::default()
            },
            CharacterInputReactions {
                press_attack: Some(InputReaction::SequenceNameString(
                    SequenceNameString::String(String::from("custom_sequence_1")),
                )),
                release_attack: Some(InputReaction::Multiple(InputReactionMultiple::new(vec![
                    InputReactionSingle {
                        next: SequenceNameString::String(String::from("custom_sequence_2")),
                        events: InputReactionAppEvents::default(),
                        requirements: vec![],
                    },
                ]))),
                hold_jump: Some(InputReaction::Single(InputReactionSingle {
                    next: SequenceNameString::String(String::from("custom_sequence_3")),
                    events: InputReactionAppEvents::default(),
                    requirements: vec![],
                })),
                ..Default::default()
            }, // kcov-ignore
        )];

        let character_input_reactions = CharacterInputReactions {
            press_defend: Some(InputReaction::SequenceNameString(
                SequenceNameString::String(String::from("custom_sequence_4")),
            )),
            ..Default::default()
        };
        CharacterSequence::new(
            ObjectSequence {
                next: SequenceEndTransition::SequenceName(SequenceNameString::String(
                    String::from("custom_sequence_1"),
                )),
                frames,
                ..Default::default()
            },
            Some(character_input_reactions),
        )
    }

    fn empty_sequence() -> CharacterSequence {
        let frames = vec![];
        CharacterSequence::new(
            ObjectSequence {
                next: SequenceEndTransition::SequenceName(SequenceNameString::Name(
                    CharacterSequenceName::Stand,
                )),
                frames,
                ..Default::default()
            },
            None,
        )
    }
}

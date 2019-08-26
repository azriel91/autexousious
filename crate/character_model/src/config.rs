//! Contains the types that represent the configuration on disk.

pub use self::{
    character_control_transitions::CharacterControlTransitions,
    character_definition::{CharacterDefinition, CharacterDefinitionHandle},
    character_frame::CharacterFrame,
    character_sequence::CharacterSequence,
    character_sequence_name::CharacterSequenceName,
    character_sequence_name_string::CharacterSequenceNameString,
    control_transition_requirement::ControlTransitionRequirement,
    control_transition_requirement_params::ControlTransitionRequirementParams,
};

mod character_control_transitions;
mod character_definition;
mod character_frame;
mod character_sequence;
mod character_sequence_name;
mod character_sequence_name_string;
mod control_transition_requirement;
mod control_transition_requirement_params;

#[cfg(test)]
mod test {
    use charge_model::config::ChargePoints;
    use collision_model::config::{Body, Interactions};
    use indexmap::IndexMap;
    use object_model::{
        config::{ObjectDefinition, ObjectFrame, ObjectSequence},
        play::{HealthPoints, SkillPoints},
    };
    use sequence_model::config::{
        ControlTransition, ControlTransitionMultiple, ControlTransitionSingle,
        SequenceEndTransition, SequenceNameString, Wait,
    };
    use serde_yaml;
    use shape_model::Volume;
    use spawn_model::config::Spawns;
    use sprite_model::config::SpriteRef;

    use crate::config::{
        CharacterControlTransitions, CharacterDefinition, CharacterFrame, CharacterSequence,
        CharacterSequenceName, ControlTransitionRequirement,
    };

    const OBJECT_YAML: &str = "\
sequences:
  stand:
    next: 'walk'
    transitions: { press_defend: 'stand_attack_1' }
    frames:
      - wait: 5
        sprite: { sheet: 1, index: 3 }
        body: [{ box: { x: 25, y: 11, w: 31, h: 68 } }]
        transitions:
          press_attack: 'stand_attack_0'
          release_attack:
            - { next: 'walk', requirements: [{ charge: 90 }] }
            - { next: 'run', requirements: [{ sp: 50 }] }
            - { next: 'run_stop', requirements: [{ hp: 30 }] }
          hold_jump: { next: 'jump' }
";

    #[test]
    fn deserialize_character_definition() {
        let char_definition = serde_yaml::from_str::<CharacterDefinition>(OBJECT_YAML)
            .expect("Failed to deserialize character definition.");

        let frames = vec![CharacterFrame::new(
            ObjectFrame::new(
                Wait::new(5),
                SpriteRef::new(1, 3),
                Body::new(vec![Volume::Box {
                    x: 25,
                    y: 11,
                    z: 0,
                    w: 31,
                    h: 68,
                    d: 26,
                }]),
                Interactions::default(),
                Spawns::default(),
            ),
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
                            requirements: vec![ControlTransitionRequirement::Sp(SkillPoints::new(
                                50,
                            ))],
                        },
                        ControlTransitionSingle {
                            next: SequenceNameString::Name(CharacterSequenceName::RunStop),
                            requirements: vec![ControlTransitionRequirement::Hp(
                                HealthPoints::new(30),
                            )],
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

        let character_control_transitions = CharacterControlTransitions {
            press_defend: Some(ControlTransition::SequenceNameString(
                SequenceNameString::Name(CharacterSequenceName::StandAttack1),
            )),
            ..Default::default()
        };
        let sequence = CharacterSequence::new(
            ObjectSequence::new(
                SequenceEndTransition::SequenceName(SequenceNameString::Name(
                    CharacterSequenceName::Walk,
                )),
                frames,
            ),
            Some(character_control_transitions),
        );
        let mut sequences = IndexMap::new();
        sequences.insert(
            SequenceNameString::Name(CharacterSequenceName::Stand),
            sequence,
        );
        let object_definition = ObjectDefinition::new(sequences);
        let expected = CharacterDefinition {
            object_definition,
            ..Default::default()
        };
        assert_eq!(expected, char_definition);
    }
}

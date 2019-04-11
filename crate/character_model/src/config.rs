//! Contains the types that represent the configuration on disk.

pub use self::{
    character_control_transition::CharacterControlTransition,
    character_definition::CharacterDefinition, character_frame::CharacterFrame,
    character_sequence::CharacterSequence, character_sequence_id::CharacterSequenceId,
    control_transition_requirement::ControlTransitionRequirement,
};

mod character_control_transition;
mod character_definition;
mod character_frame;
mod character_sequence;
mod character_sequence_id;
mod control_transition_requirement;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use collision_model::config::{Body, Interactions};
    use object_model::{
        config::{ObjectDefinition, ObjectFrame, ObjectSequence},
        play::{ChargePoints, HealthPoints, SkillPoints},
    };
    use sequence_model::config::{ControlTransition, Wait};
    use shape_model::Volume;
    use sprite_model::config::SpriteRef;
    use toml;

    use crate::config::{
        CharacterControlTransition, CharacterDefinition, CharacterFrame, CharacterSequence,
        CharacterSequenceId, ControlTransitionRequirement,
    };

    const OBJECT_TOML: &str = r#"
        [sequences.stand]
          next = "walk"

          [[sequences.stand.frames]]
            wait = 5
            sprite = { sheet = 1, index = 3 }
            body = [{ box = { x = 25, y = 11, w = 31, h = 68 } }]
            transitions = [
              { press_attack = "stand_attack" },
              { release_attack = "walk", charge = 90 },
              { release_attack = "run", sp = 50 },
              { release_attack = "run_stop", hp = 30 },
              { hold_jump = "jump" },
            ]
    "#;

    #[test]
    fn deserialize_character_definition() {
        let char_definition = toml::from_str::<CharacterDefinition>(OBJECT_TOML)
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
            ),
            vec![
                CharacterControlTransition::new(
                    ControlTransition::PressAttack(CharacterSequenceId::StandAttack),
                    ControlTransitionRequirement::default(),
                ),
                CharacterControlTransition::new(
                    ControlTransition::ReleaseAttack(CharacterSequenceId::Walk),
                    ControlTransitionRequirement {
                        charge: ChargePoints::new(90),
                        ..Default::default()
                    },
                ),
                CharacterControlTransition::new(
                    ControlTransition::ReleaseAttack(CharacterSequenceId::Run),
                    ControlTransitionRequirement {
                        sp: SkillPoints::new(50),
                        ..Default::default()
                    },
                ),
                CharacterControlTransition::new(
                    ControlTransition::ReleaseAttack(CharacterSequenceId::RunStop),
                    ControlTransitionRequirement {
                        hp: HealthPoints::new(30),
                        ..Default::default()
                    },
                ),
                CharacterControlTransition::new(
                    ControlTransition::HoldJump(CharacterSequenceId::Jump),
                    ControlTransitionRequirement::default(),
                ),
            ],
        )];
        let sequence =
            CharacterSequence::new(ObjectSequence::new(Some(CharacterSequenceId::Walk), frames));
        let mut sequences = HashMap::new();
        sequences.insert(CharacterSequenceId::Stand, sequence);
        let object_definition = ObjectDefinition::new(sequences);
        let expected = CharacterDefinition::new(object_definition);
        assert_eq!(expected, char_definition);
    }
}

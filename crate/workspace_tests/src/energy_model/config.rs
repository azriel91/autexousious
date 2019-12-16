mod energy_sequence;

#[cfg(test)]
mod test {
    use collision_model::config::Body;
    use indexmap::IndexMap;
    use object_model::config::{ObjectDefinition, ObjectFrame, ObjectSequence};
    use sequence_model::config::{Sequence, SequenceEndTransition, SequenceNameString, Wait};
    use serde_yaml;
    use shape_model::Volume;
    use sprite_model::config::SpriteRef;

    use energy_model::config::{EnergyDefinition, EnergyFrame, EnergySequence, EnergySequenceName};

    const OBJECT_YAML: &str = r#"---
sequences:
  hover:
    next: "hover"
    frames:
      - wait: 5
        sprite: { sheet: 1, index: 3 }
        body: [{ box: { x: 25, y: 11, w: 31, h: 68 } }]
"#;

    #[test]
    fn deserialize_energy_definition() {
        let char_definition = serde_yaml::from_str::<EnergyDefinition>(OBJECT_YAML)
            .expect("Failed to deserialize `EnergyDefinition`.");

        let frames = vec![EnergyFrame::new(ObjectFrame {
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
        })];
        let sequence = EnergySequence::new(ObjectSequence {
            sequence: Sequence {
                next: SequenceEndTransition::SequenceName(SequenceNameString::Name(
                    EnergySequenceName::Hover,
                )),
                frames,
            },
            ..Default::default()
        });
        let mut sequences = IndexMap::new();
        sequences.insert(
            SequenceNameString::Name(EnergySequenceName::Hover),
            sequence,
        );
        let object_definition = ObjectDefinition::new(sequences);
        let expected = EnergyDefinition::new(object_definition);
        assert_eq!(expected, char_definition);
    }
}

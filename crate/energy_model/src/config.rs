//! Contains the types that represent the configuration on disk.

pub use self::{
    energy_definition::{EnergyDefinition, EnergyDefinitionHandle},
    energy_frame::EnergyFrame,
    energy_sequence::EnergySequence,
    energy_sequence_id::EnergySequenceId,
};

mod energy_definition;
mod energy_frame;
mod energy_sequence;
mod energy_sequence_id;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use collision_model::config::{Body, Interactions};
    use object_model::config::{ObjectDefinition, ObjectFrame, ObjectSequence};
    use sequence_model::config::Wait;
    use shape_model::Volume;
    use sprite_model::config::SpriteRef;
    use toml;

    use crate::config::{EnergyDefinition, EnergyFrame, EnergySequence, EnergySequenceId};

    const OBJECT_TOML: &str = r#"
        [sequences.hover]
          next = "hover"

          [[sequences.hover.frames]]
            wait = 5
            sprite = { sheet = 1, index = 3 }
            body = [{ box = { x = 25, y = 11, w = 31, h = 68 } }]
    "#;

    #[test]
    fn deserialize_energy_definition() {
        let char_definition = toml::from_str::<EnergyDefinition>(OBJECT_TOML)
            .expect("Failed to deserialize `EnergyDefinition`.");

        let frames = vec![EnergyFrame::new(ObjectFrame::new(
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
        ))];
        let sequence =
            EnergySequence::new(ObjectSequence::new(Some(EnergySequenceId::Hover), frames));
        let mut sequences = HashMap::new();
        sequences.insert(EnergySequenceId::Hover, sequence);
        let object_definition = ObjectDefinition::new(sequences);
        let expected = EnergyDefinition::new(object_definition);
        assert_eq!(expected, char_definition);
    }
}

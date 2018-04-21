//! Configuration types for `Character`s.

pub use self::character_definition::CharacterDefinition;
pub use self::sequence_id::SequenceId;

mod character_definition;
mod sequence_id;

#[cfg(test)]
mod test {
    use toml;

    use super::{CharacterDefinition, SequenceId};
    use config::object::sequence::Frame;
    use config::object::{ObjectDefinition, Sequence};

    const OBJECT_TOML: &str = r#"
        [[sequences]]
          id = "Standing"
          next = "Walking"
          frames = [
            { sheet = 0, sprite = 4, wait = 2 },
            { sheet = 0, sprite = 5, wait = 2 },
            { sheet = 1, sprite = 6, wait = 1 },
            { sheet = 1, sprite = 7, wait = 1 },
            { sheet = 0, sprite = 6, wait = 2 },
            { sheet = 0, sprite = 5, wait = 2 },
          ]
    "#;

    #[test]
    fn deserialize_character_definition() {
        let char_definition = toml::from_str::<CharacterDefinition>(OBJECT_TOML)
            .expect("Failed to deserialize character definition.");

        let frames = vec![
            Frame::new(0, 4, 2),
            Frame::new(0, 5, 2),
            Frame::new(1, 6, 1),
            Frame::new(1, 7, 1),
            Frame::new(0, 6, 2),
            Frame::new(0, 5, 2),
        ];
        let sequence = Sequence::new(SequenceId::Standing, SequenceId::Walking, frames);
        let object_definition = ObjectDefinition::new(vec![sequence]);
        let expected = CharacterDefinition::new(object_definition);
        assert_eq!(expected, char_definition);
    }
}

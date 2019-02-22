//! Contains the types that represent the configuration on disk.

pub use self::{
    character_definition::CharacterDefinition, character_sequence_id::CharacterSequenceId,
};

mod character_definition;
mod character_sequence_id;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use collision_model::config::{Body, Interactions};
    use object_model::config::object::{ObjectDefinition, ObjectFrame, Sequence};
    use sprite_model::config::SpriteRef;
    use toml;

    use super::{CharacterDefinition, CharacterSequenceId};

    const OBJECT_TOML: &str = r#"
        [sequences.stand]
          next = "walk"
          frames = [
            { wait = 2, sprite = { sheet = 0, index = 4 } },
            { wait = 2, sprite = { sheet = 0, index = 5 } },
            { wait = 1, sprite = { sheet = 1, index = 6 } },
            { wait = 1, sprite = { sheet = 1, index = 7 } },
            { wait = 2, sprite = { sheet = 0, index = 6 } },
            { wait = 2, sprite = { sheet = 0, index = 5 } },
          ]
    "#;

    #[test]
    fn deserialize_character_definition() {
        let char_definition = toml::from_str::<CharacterDefinition>(OBJECT_TOML)
            .expect("Failed to deserialize character definition.");

        let frames = vec![
            ObjectFrame::new(
                2,
                SpriteRef::new(0, 4),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                2,
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                1,
                SpriteRef::new(1, 6),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                1,
                SpriteRef::new(1, 7),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                2,
                SpriteRef::new(0, 6),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                2,
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
            ),
        ];
        let sequence = Sequence::new(Some(CharacterSequenceId::Walk), frames);
        let mut sequences = HashMap::new();
        sequences.insert(CharacterSequenceId::Stand, sequence);
        let object_definition = ObjectDefinition::new(sequences);
        let expected = CharacterDefinition::new(object_definition);
        assert_eq!(expected, char_definition);
    }
}

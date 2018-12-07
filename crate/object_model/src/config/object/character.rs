//! Configuration types for `Character`s.

pub use self::character_definition::CharacterDefinition;
pub use self::character_sequence_id::CharacterSequenceId;

mod character_definition;
mod character_sequence_id;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use collision_model::config::{BodyFrame, InteractionFrame};
    use sprite_model::config::SpriteFrame;
    use toml;

    use super::{CharacterDefinition, CharacterSequenceId};
    use crate::config::object::sequence::ObjectFrame;
    use crate::config::object::{ObjectDefinition, Sequence};

    const OBJECT_TOML: &str = r#"
        [sequences.stand]
          next = "walk"
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
            ObjectFrame::new(
                SpriteFrame::new(0, 4, 2),
                BodyFrame::default(),
                InteractionFrame::default(),
            ),
            ObjectFrame::new(
                SpriteFrame::new(0, 5, 2),
                BodyFrame::default(),
                InteractionFrame::default(),
            ),
            ObjectFrame::new(
                SpriteFrame::new(1, 6, 1),
                BodyFrame::default(),
                InteractionFrame::default(),
            ),
            ObjectFrame::new(
                SpriteFrame::new(1, 7, 1),
                BodyFrame::default(),
                InteractionFrame::default(),
            ),
            ObjectFrame::new(
                SpriteFrame::new(0, 6, 2),
                BodyFrame::default(),
                InteractionFrame::default(),
            ),
            ObjectFrame::new(
                SpriteFrame::new(0, 5, 2),
                BodyFrame::default(),
                InteractionFrame::default(),
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

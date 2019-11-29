use character_model::config::CharacterDefinition;
use lazy_static::lazy_static;

lazy_static! {
    /// Default `CharacterDefinition` with control transitions.
    pub static ref CHARACTER_INPUT_REACTIONS_DEFAULT: CharacterDefinition = {
        let definition_yaml = include_str!("character_input_reactions_default.yaml");
        serde_yaml::from_str::<CharacterDefinition>(definition_yaml)
            .expect("Failed to deserialize `character_input_reactions_default.yaml`.")
    };
}

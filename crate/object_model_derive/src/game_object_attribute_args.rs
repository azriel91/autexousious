use syn::{
    parse::{Parse, ParseStream},
    Path, Result,
};

/// Parses the `Path` for the type to use as a `GameObject`'s `SequenceId`.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
///     #[game_object(CharacterSequenceId)]
///                   ^^^^^^^^^^^^^^^^^^^
#[derive(Debug)]
pub struct GameObjectAttributeArgs {
    /// The sequence ID for the `GameObject`.
    pub sequence_id: Path,
}

impl Parse for GameObjectAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let sequence_id = input.parse()?;
        Ok(GameObjectAttributeArgs { sequence_id })
    }
}

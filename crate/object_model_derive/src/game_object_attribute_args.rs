use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Path, Result, Token,
};

/// Parses the `Path` for the type to use as a `GameObject`'s `SequenceId`.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[game_object(CharacterSequenceId)]
///               ^^^^^^^^^^^^^^^^^^^
/// ```
#[derive(Debug)]
pub struct GameObjectAttributeArgs {
    /// The sequence ID for the `GameObject`.
    pub sequence_id: Path,
    /// Type that `impl GameObjectDefinition`, e.g. `CharacterDefinition`.
    pub object_definition: Option<Path>,
}

impl Parse for GameObjectAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let paths = Punctuated::<Path, Token![,]>::parse_terminated(input)?;
        let mut paths_iter = paths.into_iter();

        let sequence_id = paths_iter
            .next()
            .ok_or_else(|| Error::new(Span::call_site(), "Must provide `SequenceId` type."))?;
        let object_definition = paths_iter.next();

        Ok(GameObjectAttributeArgs {
            sequence_id,
            object_definition,
        })
    }
}

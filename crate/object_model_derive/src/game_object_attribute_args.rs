use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token, Variant,
};

/// https://docs.rs/syn/latest/syn/macro.custom_keyword.html
mod kw {
    syn::custom_keyword!(sequence);
    syn::custom_keyword!(definition);
    syn::custom_keyword!(object_type);
}

/// Parses the `Path` for the type to use as a `GameObject`'s `SequenceId`.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[game_object(CharacterSequenceId)]
///               ^^^^^^^^^^^^^^^^^^^
/// ```
///
/// The following parameters are optional:
///
/// * `sequence`: Specifies the game object sequence type.
/// * `definition`: Specifies the game object definition type.
/// * `object_type`: Specifies the object type variant.
///
/// ```rust,ignore
/// #[game_object(
///     CharacterSequenceId,
///     sequence = CharacterSequence,
///     definition = CharacterDefinition,
///     object_type = ObjectType::Character,
/// )]
/// ```
#[derive(Debug)]
pub struct GameObjectAttributeArgs {
    /// The sequence ID for the `GameObject`.
    pub sequence_id: Path,
    /// Type that `impl GameObjectSequence`, e.g. `CharacterSequence`.
    pub sequence_type: Option<Path>,
    /// Type that `impl GameObjectDefinition`, e.g. `CharacterDefinition`.
    pub object_definition: Option<Path>,
    /// `ObjectType` variant, e.g. `ObjectType::Character`.
    pub object_type: Option<Variant>,
}

impl Parse for GameObjectAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let sequence_id = input.parse()?;
        let mut sequence_type = None;
        let mut object_definition = None;
        let mut object_type = None;

        let mut comma: Option<Token![,]> = input.parse()?;
        while comma.is_some() && !input.is_empty() {
            if input.peek(kw::sequence) {
                input
                    .parse::<kw::sequence>()
                    .map_err(|_| input.error("Impossible: peek sequence"))?;
                input
                    .parse::<Token![=]>()
                    .map_err(|_| input.error("Expected `=` after `sequence` parameter name."))?;
                sequence_type = Some(
                    input
                        .parse()
                        .map_err(|_| input.error("Expected path to `GameObjectSequence` type."))?,
                );

                comma = input.parse()?;
            } else if input.peek(kw::definition) {
                input
                    .parse::<kw::definition>()
                    .map_err(|_| input.error("Impossible: peek definition"))?;
                input
                    .parse::<Token![=]>()
                    .map_err(|_| input.error("Expected `=` after `definition` parameter name."))?;
                object_definition =
                    Some(input.parse().map_err(|_| {
                        input.error("Expected path to `GameObjectDefinition` type.")
                    })?);

                comma = input.parse()?;
            } else if input.peek(kw::object_type) {
                input
                    .parse::<kw::object_type>()
                    .map_err(|_| input.error("Impossible: peek object_type"))?;
                input
                    .parse::<Token![=]>()
                    .map_err(|_| input.error("Expected `=` after `object_type` parameter name."))?;
                object_type = Some(
                    input
                        .parse()
                        .map_err(|_| input.error("Expected path to `ObjectType` variant."))?,
                );

                comma = input.parse()?;
            }
        }

        Ok(GameObjectAttributeArgs {
            sequence_id,
            sequence_type,
            object_definition,
            object_type,
        })
    }
}

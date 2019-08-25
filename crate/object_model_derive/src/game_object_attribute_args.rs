use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token, Variant,
};

/// https://docs.rs/syn/latest/syn/macro.custom_keyword.html
mod kw {
    syn::custom_keyword!(sequence_name);
    syn::custom_keyword!(sequence);
    syn::custom_keyword!(definition);
    syn::custom_keyword!(object_type);
}

/// Parses the `Path` for the type to use as a `GameObject`'s `SequenceName`.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[game_object(CharacterSequenceName)]
///               ^^^^^^^^^^^^^^^^^^^^^
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
///     sequence_name = CharacterSequenceName,
///     sequence = CharacterSequence,
///     definition = CharacterDefinition,
///     object_type = ObjectType::Character,
/// )]
/// ```
#[derive(Debug)]
pub struct GameObjectAttributeArgs {
    /// The sequence name for the `GameObject`.
    pub sequence_name: Option<Path>,
    /// Type that `impl GameObjectSequence`, e.g. `CharacterSequence`.
    pub sequence_type: Option<Path>,
    /// Type that `impl GameObjectDefinition`, e.g. `CharacterDefinition`.
    pub object_definition: Option<Path>,
    /// `ObjectType` variant, e.g. `ObjectType::Character`.
    pub object_type: Option<Variant>,
}

impl Parse for GameObjectAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut sequence_name = None;
        let mut sequence_type = None;
        let mut object_definition = None;
        let mut object_type = None;

        while !input.is_empty() {
            macro_rules! parse_param {
                ($var:ident, $keyword:ident, $param_type:path) => {
                    if input.peek(kw::$keyword) {
                        input
                            .parse::<kw::$keyword>()
                            .map_err(|_| input.error(concat!("Impossible: parse after peek `", stringify!($keyword), "`.")))?;
                        input
                            .parse::<Token![=]>()
                            .map_err(|_| input.error(concat!("Expected `=` after `", stringify!($keyword),"` parameter name.")))?;
                        $var = Some(
                            input
                                .parse()
                                .map_err(|_| input.error(concat!("Expected path to `", stringify!($param_type) ,"`.")))?,
                        );

                        let comma = input.parse::<Option<Token![,]>>()?;
                        if comma.is_none() {
                            break;
                        }

                        continue;
                    }
                };
            }
            parse_param!(sequence_name, sequence_name, SequenceName);
            parse_param!(sequence_type, sequence, GameObjectSequence);
            parse_param!(object_definition, definition, GameObjectDefinition);
            parse_param!(object_type, object_type, ObjectType);

            break;
        }

        Ok(GameObjectAttributeArgs {
            sequence_name,
            sequence_type,
            object_definition,
            object_type,
        })
    }
}

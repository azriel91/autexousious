use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token,
};

/// https://docs.rs/syn/latest/syn/macro.custom_keyword.html
mod kw {
    syn::custom_keyword!(component_owned);
}

/// Parses the `Path` for the component type in the component sequence.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[component_sequence(Wait)]
///                      ^^^^
/// ```
///
/// The following parameters are optional:
///
/// * `component_owned`: Path to the function to map a borrowed component to an owned component.
///
/// ```rust,ignore
/// #[component_sequence(Wait, component_owned = std::ops::Deref::deref)]
/// ```
#[derive(Debug)]
pub struct ComponentSequenceAttributeArgs {
    /// The component type of the `ComponentSequence`.
    pub component_path: Path,
    /// Function to map a borrowed component to an owned component. i.e. `fn(&C) -> C`.
    pub component_owned_fn: Option<Path>,
}

impl Parse for ComponentSequenceAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let component_path = input.parse()?;
        let mut component_owned_fn = None;

        let mut comma: Option<Token![,]> = input.parse()?;
        while comma.is_some() && !input.is_empty() {
            if input.peek(kw::component_owned) {
                input
                    .parse::<kw::component_owned>()
                    .map_err(|_| input.error("Impossible: peek component_owned"))?;
                input.parse::<Token![=]>().map_err(|_| {
                    input.error("Expected `=` after `component_owned` parameter name.")
                })?;
                component_owned_fn = Some(
                    input
                        .parse()
                        .map_err(|_| input.error("Expected path to `GameObjectSequence` type."))?,
                );

                comma = input.parse()?;
            }
        }

        Ok(ComponentSequenceAttributeArgs {
            component_path,
            component_owned_fn,
        })
    }
}

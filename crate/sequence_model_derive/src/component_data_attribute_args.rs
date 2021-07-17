use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token,
};

/// https://docs.rs/syn/latest/syn/macro.custom_keyword.html
mod kw {
    syn::custom_keyword!(copy);
    syn::custom_keyword!(to_owned);
}

/// Parses the `Path` for the component type in component data.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[frame_component_data(Wait)]
///                        ^^^^
///
/// // or one of:
/// #[frame_component_data(Copyable, copy)]
/// #[frame_component_data(CustomToOwned, to_owned = std::ops::Deref::deref)]
/// ```
///
/// The following parameters are optional:
///
/// * `copy`: Indicates that the component is `Copy`.
/// * `to_owned`: Path to the function to map a borrowed component to an owned
///   component.
///
/// ```rust,ignore
/// #[frame_component_data(Wait, to_owned = std::ops::Deref::deref)]
/// ```
#[derive(Debug)]
pub struct ComponentDataAttributeArgs {
    /// The component type of the `FrameComponentData`.
    pub component_path: Path,
    /// Whether the type is copy.
    pub component_copy: bool,
    /// Function to map a borrowed component to an owned component. i.e. `fn(&C)
    /// -> C`.
    pub to_owned_fn: Option<Path>,
}

impl Parse for ComponentDataAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let component_path = input.parse()?;
        let mut component_copy = false;
        let mut to_owned_fn = None;

        let mut comma: Option<Token![,]> = input.parse()?;
        while comma.is_some() && !input.is_empty() {
            if input.peek(kw::to_owned) {
                input
                    .parse::<kw::to_owned>()
                    .map_err(|_| input.error("Impossible: peek to_owned"))?;
                input
                    .parse::<Token![=]>()
                    .map_err(|_| input.error("Expected `=` after `to_owned` parameter name."))?;
                to_owned_fn = Some(
                    input
                        .parse()
                        .map_err(|_| input.error("Expected path to a `to_owned` function."))?,
                );

                comma = input.parse()?;
            } else if input.peek(kw::copy) {
                input
                    .parse::<kw::copy>()
                    .map_err(|_| input.error("Impossible: peek copy"))?;

                component_copy = true;
                comma = input.parse()?;
            }
        }

        Ok(ComponentDataAttributeArgs {
            component_path,
            component_copy,
            to_owned_fn,
        })
    }
}

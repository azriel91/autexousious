use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token,
};

use crate::ComponentDataAttributeArgs;

/// https://docs.rs/syn/latest/syn/macro.custom_keyword.html
mod kw {
    syn::custom_keyword!(copy);
    syn::custom_keyword!(to_owned);
}

/// Parses the `SequenceId` and `Path` for the component type in the sequence component data.
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
/// ```rust,ignore
/// #[sequence_component_data(MagicSequenceId, Wait)]
///                           ^^^^^^^^^^^^^^^^^^^^^
///
/// // or one of:
/// #[sequence_component_data(MagicSequenceId, Copyable, copy)]
/// #[sequence_component_data(MagicSequenceId, CustomToOwned, to_owned = std::ops::Deref::deref)]
/// ```
///
/// The following parameters are optional:
///
/// * `copy`: Indicates that the component is `Copy`.
/// * `to_owned`: Path to the function to map a borrowed component to an owned component.
///
/// ```rust,ignore
/// #[sequence_component_data(Wait, to_owned = std::ops::Deref::deref)]
/// ```
#[derive(Debug)]
pub struct SequenceComponentDataAttributeArgs {
    /// The sequence ID type of the `SequenceComponentData`.
    pub sequence_id_path: Path,
    /// The component type of the `SequenceComponentData`.
    pub component_data_attribute_args: ComponentDataAttributeArgs,
}

impl Parse for SequenceComponentDataAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let sequence_id_path = input.parse()?;
        let _comma: Option<Token![,]> = input.parse()?;
        let component_data_attribute_args = input.parse()?;

        Ok(SequenceComponentDataAttributeArgs {
            sequence_id_path,
            component_data_attribute_args,
        })
    }
}

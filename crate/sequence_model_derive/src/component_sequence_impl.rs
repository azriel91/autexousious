use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputDeriveExt, DeriveInputStructExt, FieldsUnnamedAppend};
use quote::quote;
use syn::{parse_quote, DeriveInput, FieldsUnnamed, Path};

use crate::component_sequence_attribute_args::ComponentSequenceAttributeArgs;

/// Generates the `ComponentSequence` implementation.
pub fn component_sequence_impl(
    mut ast: DeriveInput,
    args: ComponentSequenceAttributeArgs,
) -> TokenStream {
    let component_path = args.component_path;
    let to_owned_fn_impl = {
        if args.component_copy {
            quote! {
                fn to_owned(component: &Self::Component) -> Self::Component {
                    *component
                }
            }
        } else {
            let to_owned_fn = args
                .to_owned_fn
                .unwrap_or_else(|| parse_quote!(std::clone::Clone::clone));

            quote! {
                fn to_owned(component: &Self::Component) -> Self::Component {
                    #to_owned_fn(component)
                }
            }
        }
    };

    ast.assert_fields_unit();

    derive_append(&mut ast);
    fields_append(&mut ast, &component_path);

    let type_name = &ast.ident;
    let fn_new_doc = format!("Returns a new `{}`.", type_name);

    let token_stream_2 = quote! {
        #[derive(
            asset_derive::Asset,
            derive_deref::Deref,
            derive_deref::DerefMut,
            typename_derive::TypeName,
        )]
        #ast

        impl #type_name {
            #[doc = #fn_new_doc]
            pub fn new(sequence: Vec<#component_path>) -> Self {
                #type_name(
                    sequence_model_spi::loaded::ComponentSequence::<#component_path>::new(sequence)
                )
            }
        }

        // Manually implement `Default` because the component type may not, and the `Default` derive
        // imposes a `Default` bound on type parameters.
        impl Default for #type_name {
            fn default() -> Self {
                #type_name(
                    sequence_model_spi::loaded::ComponentSequence::<#component_path>::new(
                        Vec::default()
                    )
                )
            }
        }

        impl sequence_model_spi::loaded::ComponentDataExt for #type_name {
            type Component = #component_path;

            #to_owned_fn_impl
        }
    };

    TokenStream::from(token_stream_2)
}

fn derive_append(ast: &mut DeriveInput) {
    let derives = parse_quote!(Clone, Debug, PartialEq);

    ast.append_derives(derives);
}

fn fields_append(ast: &mut DeriveInput, component_path: &Path) {
    let component_name = &component_path
        .segments
        .last()
        .expect("Expected `Path` last segment to exist.")
        .value()
        .ident;
    let doc_string = format!("The chain of `{}` values.", component_name);
    let fields: FieldsUnnamed = parse_quote! {
        (#[doc = #doc_string] pub sequence_model_spi::loaded::ComponentSequence<#component_path>)
    };

    ast.append_unnamed(fields);
}

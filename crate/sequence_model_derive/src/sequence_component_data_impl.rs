use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputExt, DeriveInputStructExt, FieldsUnnamedAppend};
use quote::quote;
use syn::{parse_quote, DeriveInput, FieldsUnnamed, Path};

use crate::{to_owned_fn_impl, ComponentDataAttributeArgs};

/// Generates the `SequenceComponentData` implementation.
pub fn sequence_component_data_impl(
    mut ast: DeriveInput,
    args: ComponentDataAttributeArgs,
) -> TokenStream {
    let ComponentDataAttributeArgs {
        component_path,
        component_copy,
        to_owned_fn,
    } = args;

    ast.assert_fields_unit();
    derive_append(&mut ast);
    fields_append(&mut ast, &component_path);

    let type_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let to_owned_fn_impl = to_owned_fn_impl(component_copy, to_owned_fn);

    let fn_new_doc = format!("Returns a new `{}`.", type_name);

    let token_stream_2 = quote! {
        #ast

        impl #impl_generics #type_name #ty_generics #where_clause {
            #[doc = #fn_new_doc]
            pub fn new(sequence: std::vec::Vec<#component_path>) -> Self {
                #type_name(
                    sequence_model_spi::loaded::SequenceComponentData::<
                        #component_path
                    >::new(sequence)
                )
            }
        }

        // Manually implement `Default` because the component type may not, and the `Default` derive
        // imposes a `Default` bound on type parameters.
        impl #impl_generics Default for #type_name #ty_generics #where_clause {
            fn default() -> Self {
                #type_name(
                    sequence_model_spi::loaded::SequenceComponentData::<
                        #component_path
                    >::new(std::vec::Vec::default())
                )
            }
        }

        impl #impl_generics sequence_model_spi::loaded::ComponentDataExt for
            #type_name #ty_generics #where_clause
        {
            type Component = #component_path;

            fn new(sequence: std::vec::Vec<#component_path>) -> Self {
                #type_name::new(sequence)
            }

            #to_owned_fn_impl
        }
    };

    TokenStream::from(token_stream_2)
}

fn derive_append(ast: &mut DeriveInput) {
    let derives = parse_quote!(
        derive_deref::Deref,
        derive_deref::DerefMut,
        typename_derive::TypeName,
        Clone,
        Debug,
        PartialEq
    );

    ast.append_derives(derives);
}

fn fields_append(ast: &mut DeriveInput, component_path: &Path) {
    let component_name = &component_path
        .segments
        .last()
        .expect("Expected `Path` last segment to exist.")
        .ident;
    let doc_string = format!("The chain of `{}` values.", component_name);
    let fields: FieldsUnnamed = parse_quote! {
        (#[doc = #doc_string]
        pub sequence_model_spi::loaded::SequenceComponentData<#component_path>)
    };

    ast.append_unnamed(fields);
}

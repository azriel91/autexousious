#![recursion_limit = "128"]

//! Provides the `#[derive(VariantStruct)]` macro to generate a struct for each enum variant.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Field, Fields};

/// Attributes that should be copied across.
const ATTRIBUTES_TO_COPY: &[&str] = &["doc", "cfg", "allow", "deny"];

/// Derives a struct for each enum variant.
///
/// Struct fields including their attributes are copied over.
#[proc_macro_derive(VariantStruct)]
pub fn variant_struct_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = &ast.ident;
    let vis = &ast.vis;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let data_enum = data_enum(&ast);
    let variants = &data_enum.variants;

    let mut struct_declarations = proc_macro2::TokenStream::new();
    let struct_declarations_iter = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let attrs = variant
            .attrs
            .iter()
            .filter(|attribute| {
                ATTRIBUTES_TO_COPY
                    .iter()
                    .any(|attr_to_copy| attribute.path.is_ident(attr_to_copy))
            })
            .collect::<Vec<&Attribute>>();
        let fields = &variant.fields;

        // Need to attach visibility modifier to fields.
        let fields_with_vis = fields
            .iter()
            .cloned()
            .map(|mut field| {
                field.vis = vis.clone();
                field
            })
            .collect::<Vec<Field>>();

        let data_struct = match fields {
            Fields::Unit => quote! {
                struct #variant_name;
            },
            Fields::Unnamed(..) => {
                quote! {
                    struct #variant_name #ty_generics (#(#fields_with_vis,)*) #where_clause;
                }
            }
            Fields::Named(..) => quote! {
                struct #variant_name #ty_generics #where_clause {
                    #(#fields_with_vis,)*
                }
            },
        };

        // TODO: This generates invalid code if the type parameter is not used by this variant.
        let of_variant_impl = quote! {
            impl #impl_generics variant_struct::OfVariant for #variant_name #ty_generics
            #where_clause {
                type Enum = #enum_name;

                fn is_for(value: &#enum_name) -> bool {
                    if let #enum_name::#variant_name { .. } = value {
                        true
                    } else {
                        false
                    }
                }
            }
        };

        quote! {
            #(#attrs)*
            #vis #data_struct

            #of_variant_impl
        }
    });
    struct_declarations.extend(struct_declarations_iter);
    struct_declarations.into()
}

fn data_enum(ast: &DeriveInput) -> &DataEnum {
    if let Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("`VariantStruct` derive can only be used on an enum.");
    }
}

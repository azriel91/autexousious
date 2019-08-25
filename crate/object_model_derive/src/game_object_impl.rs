use quote::quote;
use syn::{DeriveInput, Ident, Path, Variant};

/// Generates the trait implementation for `GameObject`.
///
/// See `object_model::loaded::GameObject`.
pub fn game_object_impl(
    ast: &DeriveInput,
    object_type_variant: &Variant,
    sequence_name_type: &Path,
    sequence_type: &Path,
    object_definition_type: &Path,
    object_wrapper_name: &Ident,
    object_handle_field: &Ident,
) -> proc_macro2::TokenStream {
    let ty_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    quote! {
        impl #impl_generics object_model::loaded::GameObject for
            #ty_name #ty_generics #where_clause {
            const OBJECT_TYPE: object_type::ObjectType =
                object_type::ObjectType::#object_type_variant;

            type SequenceName = #sequence_name_type;
            type GameObjectSequence = #sequence_type;
            type Definition = #object_definition_type;
            type ObjectWrapper = #object_wrapper_name;

            fn object_handle(&self) -> &amethyst::assets::Handle<#object_wrapper_name> {
                &self.#object_handle_field
            }
        }
    }
}

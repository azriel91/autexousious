use quote::quote;
use syn::{DeriveInput, Ident, Path};

/// Generates the trait implementation for `GameObject`.
///
/// See `object_model::loaded::GameObject`.
pub fn game_object_impl(
    ast: &DeriveInput,
    sequence_id_type: &Path,
    object_handle_field_name: &Ident,
    sequence_end_transitions_field_name: &Ident,
    object_wrapper_type: &Ident,
) -> proc_macro2::TokenStream {
    let ty_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    quote! {
        impl #impl_generics object_model::loaded::GameObject<#sequence_id_type> for
            #ty_name #ty_generics #where_clause {
            type ObjectWrapper = #object_wrapper_type;

            fn object_handle(&self) -> &object_model::loaded::ObjectHandle<#sequence_id_type> {
                &self.#object_handle_field_name
            }

            fn sequence_end_transitions(&self)
            -> &object_model::loaded::SequenceEndTransitions<#sequence_id_type> {
                &self.#sequence_end_transitions_field_name
            }
        }
    }
}

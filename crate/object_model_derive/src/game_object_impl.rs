use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::util::{ast_fields, ensure_fields_named, find_field, first_type_param};

const ERR_MUST_BE_STRUCT: &str = "`GameObject` derive must be used on a struct.";
const ERR_MUST_BE_NAMED: &str = "`GameObject` derive must be used on a struct with named fields.\n\
                                 This derive does not work on tuple or unit structs.";

pub fn game_object_impl(ast: &DeriveInput) -> TokenStream {
    let ty_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = ast_fields(&ast, ERR_MUST_BE_STRUCT);
    ensure_fields_named(fields, ERR_MUST_BE_NAMED);

    let object_handle_field = find_field(fields, stringify!(ObjectHandle))
        .unwrap_or_else(|| panic!("Unable to find field with type: `ObjectHandle<_>`."));
    let object_handle_field_name = &object_handle_field.ident;
    let sequence_end_transitions_field = find_field(fields, stringify!(SequenceEndTransitions))
        .unwrap_or_else(|| panic!("Unable to find field with type: `SequenceEndTransitions<_>`."));
    let sequence_end_transitions_field_name = &sequence_end_transitions_field.ident;

    let sequence_id_type = {
        let handle_seq_id_type = first_type_param(&object_handle_field.ty).unwrap_or_else(|| {
            panic!(
                "Failed to read first type parameter of `{}` field.",
                object_handle_field_name.as_ref().unwrap()
            )
        });

        let transitions_seq_id_type = first_type_param(&sequence_end_transitions_field.ty)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to read first type parameter of `{}` field.",
                    sequence_end_transitions_field_name.as_ref().unwrap()
                )
            });

        if handle_seq_id_type != transitions_seq_id_type {
            panic!(
                "`ObjectHandle` sequence ID type: `{:?}` \
                 must match `SequenceEndTransitions` sequence ID type: `{:?}`",
                handle_seq_id_type, transitions_seq_id_type
            );
        }

        handle_seq_id_type
    };

    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let gen = quote! {
        impl #impl_generics GameObject<#sequence_id_type> for #ty_name #ty_generics #where_clause {
            fn object_handle(&self) -> &ObjectHandle<#sequence_id_type> {
                &self.#object_handle_field_name
            }

            fn sequence_end_transitions(&self) -> &SequenceEndTransitions<#sequence_id_type> {
                &self.#sequence_end_transitions_field_name
            }
        }
    };
    gen.into()
}

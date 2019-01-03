use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{punctuated::Pair, DeriveInput, Field, Ident, Path};

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

    let sequence_id_type =
        find_sequence_id_type(&object_handle_field, &sequence_end_transitions_field);

    let doc_string = {
        let last_segment = sequence_id_type
            .segments
            .last()
            .expect("Failed to get last path segment for Sequence ID type.");
        if let Pair::End(path_segment) = last_segment {
            format!(
                "Newtype for `Object<{}>`.",
                path_segment.clone().into_token_stream()
            )
        } else {
            String::from("Newtype for `Object<?>`.")
        }
    };
    let object_wrapper_type = {
        let object_wrapper_type_name = ty_name.to_string() + "ObjectWrapper";
        Ident::new(&object_wrapper_type_name, Span::call_site())
    };

    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let gen = quote! {
        use derive_deref::{Deref, DerefMut};

        #[doc = #doc_string]
        #[derive(Debug, Deref, DerefMut)]
        pub struct #object_wrapper_type(pub object_model::loaded::ObjectHandle<#sequence_id_type>);

        impl object_model::loaded::ObjectWrapper for #object_wrapper_type {
            type SequenceId = #sequence_id_type;

            fn new(object_handle: object_model::loaded::ObjectHandle<Self::SequenceId>) -> Self {
                #object_wrapper_type(object_handle)
            }
        }

        impl amethyst::assets::Asset for #object_wrapper_type {
            const NAME: &'static str = concat!(
                module_path!(),
                "::",
                stringify!(#object_wrapper_type)
            );

            type Data = Self;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;
        }

        impl From<#object_wrapper_type> for std::result::Result<
            amethyst::assets::ProcessingState<#object_wrapper_type>,
            amethyst::assets::Error
        > {
            fn from(object: #object_wrapper_type) -> std::result::Result<
                amethyst::assets::ProcessingState<#object_wrapper_type>,
                amethyst::assets::Error
            > {
                Ok(amethyst::assets::ProcessingState::Loaded(object))
            }
        }

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
    };
    gen.into()
}

fn find_sequence_id_type<'f>(
    object_handle_field: &'f Field,
    sequence_end_transitions_field: &'f Field,
) -> &'f Path {
    let object_handle_field_name = &object_handle_field.ident;
    let handle_seq_id_type = first_type_param(&object_handle_field.ty).unwrap_or_else(|| {
        panic!(
            "Failed to read first type parameter of `{}` field.",
            object_handle_field_name.as_ref().unwrap()
        )
    });

    let sequence_end_transitions_field_name = &sequence_end_transitions_field.ident;
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
}

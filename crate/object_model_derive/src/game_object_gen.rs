use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_quote, DeriveInput, Fields, FieldsNamed, Ident, Path};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs,
    game_object_impl::game_object_impl,
    object_wrapper_gen::object_wrapper_gen,
    util::{data_struct_mut, ident_concat},
};

const OBJECT_HANDLE: &str = "object_handle";
const SEQUENCE_END_TRANSITIONS: &str = "sequence_end_transitions";

const ERR_MUST_BE_STRUCT: &str = "`game_object` attribute must be used on a struct.";
const ERR_MUST_BE_UNIT_OR_NAMED: &str =
    "`game_object` attribute must be used on either a unit struct or a struct with named fields.\n\
     This derive does not work on tuple structs.";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let game_object_name = ast.ident.to_string();
    let sequence_id = &args.sequence_id;
    let object_definition_type = &args
        .object_definition
        .unwrap_or_else(|| Path::from(ident_concat(&game_object_name, "Definition")));
    let object_wrapper_name = ident_concat(&game_object_name, "ObjectWrapper");

    let object_handle_field = Ident::new(OBJECT_HANDLE, Span::call_site());
    let sequence_end_transitions_field = Ident::new(SEQUENCE_END_TRANSITIONS, Span::call_site());
    let additional_fields = object_fields_additional(
        &object_wrapper_name,
        sequence_id,
        &object_handle_field,
        &sequence_end_transitions_field,
    );

    // Add object related fields.
    let data_struct = data_struct_mut(&mut ast, ERR_MUST_BE_STRUCT);
    object_fields_gen(&mut data_struct.fields, additional_fields);

    // Generate `<Type>ObjectWrapper` newtype.
    let mut object_wrapper_impl = object_wrapper_gen(
        sequence_id,
        &object_definition_type,
        &object_wrapper_name,
        &ast.vis,
    );

    // Implement `GameObject` trait.
    let game_object_trait_impl = game_object_impl(
        &ast,
        sequence_id,
        &object_handle_field,
        &sequence_end_transitions_field,
        &object_wrapper_name,
    );
    object_wrapper_impl.extend(ast.into_token_stream());
    object_wrapper_impl.extend(game_object_trait_impl);
    TokenStream::from(object_wrapper_impl)
}

fn object_fields_gen(mut fields: &mut Fields, additional_fields: FieldsNamed) {
    match &mut fields {
        Fields::Named(ref mut fields_named) => {
            additional_fields
                .named
                .into_iter()
                .for_each(|field| fields_named.named.push(field));
        }
        Fields::Unit => *fields = Fields::from(additional_fields),
        Fields::Unnamed(_) => panic!(ERR_MUST_BE_UNIT_OR_NAMED),
    };
}

fn object_fields_additional(
    object_wrapper_name: &Ident,
    sequence_id: &Path,
    object_handle_field: &Ident,
    sequence_end_transitions_field: &Ident,
) -> FieldsNamed {
    parse_quote!({
        /// Handle to loaded object data.
        pub #object_handle_field: amethyst::assets::Handle<#object_wrapper_name>,
        /// Component sequence transitions when a sequence ends.
        pub #sequence_end_transitions_field: object_model::loaded::SequenceEndTransitions<#sequence_id>,
    })
}

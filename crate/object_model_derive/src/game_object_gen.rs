use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{ident_concat, FieldsNamedAppend};
use quote::ToTokens;
use syn::{parse_quote, DeriveInput, Ident, Path};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs, game_object_impl::game_object_impl,
    object_wrapper_gen::object_wrapper_gen,
};

const OBJECT_HANDLE: &str = "object_handle";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let game_object_ident = &ast.ident;
    let game_object_name = game_object_ident.to_string();
    let sequence_id = &args.sequence_id;
    let sequence_type = &args
        .sequence_type
        .unwrap_or_else(|| Path::from(ident_concat(&game_object_name, "Sequence")));
    let object_definition_type = &args
        .object_definition
        .unwrap_or_else(|| Path::from(ident_concat(&game_object_name, "Definition")));
    let object_type_variant = &args
        .object_type
        .unwrap_or_else(|| parse_quote!(#game_object_ident));
    let object_wrapper_name = ident_concat(&game_object_name, "ObjectWrapper");

    let object_handle_field = Ident::new(OBJECT_HANDLE, Span::call_site());

    // Add object fields.
    fields_append(&mut ast, &object_wrapper_name, &object_handle_field);

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
        object_type_variant,
        sequence_id,
        &sequence_type,
        &object_definition_type,
        &object_wrapper_name,
        &object_handle_field,
    );
    object_wrapper_impl.extend(ast.into_token_stream());
    object_wrapper_impl.extend(game_object_trait_impl);
    TokenStream::from(object_wrapper_impl)
}

fn fields_append(ast: &mut DeriveInput, object_wrapper_name: &Ident, object_handle_field: &Ident) {
    let fields = parse_quote!({
        /// Handle to loaded object data.
        pub #object_handle_field: amethyst::assets::Handle<#object_wrapper_name>,
    });

    ast.append_named(fields);
}

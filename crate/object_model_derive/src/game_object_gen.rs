use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{FieldsNamedAppend, IdentExt};
use quote::ToTokens;
use syn::{parse_quote, DeriveInput, Ident, Path};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs, game_object_impl::game_object_impl,
    object_wrapper_gen::object_wrapper_gen,
};

const OBJECT_HANDLE: &str = "object_handle";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let game_object_ident = &ast.ident;
    let sequence_id = &args
        .sequence_id
        .unwrap_or_else(|| Path::from(game_object_ident.append("SequenceId")));
    let sequence_type = &args
        .sequence_type
        .unwrap_or_else(|| Path::from(game_object_ident.append("Sequence")));
    let object_definition_type = &args
        .object_definition
        .unwrap_or_else(|| Path::from(game_object_ident.append("Definition")));
    let object_type_variant = &args
        .object_type
        .unwrap_or_else(|| parse_quote!(#game_object_ident));
    let object_wrapper_name = game_object_ident.append("ObjectWrapper");

    let object_handle_field = Ident::new(OBJECT_HANDLE, Span::call_site());

    // Add object fields.
    fields_append(&mut ast, &object_wrapper_name, &object_handle_field);

    // Generate `<Type>ObjectWrapper` newtype.
    let mut object_wrapper_gen =
        object_wrapper_gen(&object_definition_type, &object_wrapper_name, &ast.vis);

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
    object_wrapper_gen.extend(ast.into_token_stream());
    object_wrapper_gen.extend(game_object_trait_impl);
    TokenStream::from(object_wrapper_gen)
}

fn fields_append(ast: &mut DeriveInput, object_wrapper_name: &Ident, object_handle_field: &Ident) {
    let fields = parse_quote!({
        /// Handle to the object data.
        pub #object_handle_field: amethyst::assets::Handle<#object_wrapper_name>,
    });

    ast.append_named(fields);
}

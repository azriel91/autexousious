use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, Attribute, DeriveInput, Fields, FieldsNamed, Ident, Meta, NestedMeta, Path,
};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs,
    util::{data_struct_mut, meta_list_contains},
};

const ERR_MUST_BE_STRUCT: &str = "`game_object` attribute must be used on a struct.";
const ERR_MUST_BE_UNIT_OR_NAMED: &str =
    "`game_object` attribute must be used on either a unit struct or a struct with named fields.\n\
     This derive does not work on tuple structs.";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let data_struct = data_struct_mut(&mut ast, ERR_MUST_BE_STRUCT);

    object_fields_gen(&args.sequence_id, &mut data_struct.fields);
    derive_gen(&mut ast);

    ast.into_token_stream().into()
}

fn object_fields_gen(sequence_id: &Path, mut fields: &mut Fields) {
    let additional_fields = object_fields_additional(sequence_id);

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

fn object_fields_additional(sequence_id: &Path) -> FieldsNamed {
    parse_quote!({
        /// Handle to loaded object data.
        pub object_handle: object_model::loaded::ObjectHandle<#sequence_id>,
        /// Component sequence transitions when a sequence ends.
        pub sequence_end_transitions: object_model::loaded::SequenceEndTransitions<#sequence_id>,
    })
}

/// Adds the `GameObject` derive to the list of derives.
fn derive_gen(ast: &mut DeriveInput) {
    let derive_meta_list_opt = ast
        .attrs
        .iter_mut()
        // Note: `parse_meta()` returns a `Meta`, which is not referenced by the `DeriveInput`.
        .filter_map(|attr| attr.parse_meta().ok().map(|meta| (attr, meta)))
        .filter_map(|(attr, meta)| match meta {
            Meta::List(meta_list) => {
                if meta_list.ident == "derive" {
                    Some((attr, meta_list))
                } else {
                    None
                }
            }
            _ => None,
        })
        .next();

    if let Some((attr, mut derive_meta_list)) = derive_meta_list_opt {
        // Emit warning if the user derives `GameObject`, as we can do that for them.
        if meta_list_contains(&derive_meta_list, "GameObject") {
            // TODO: Emit warning, pending <https://github.com/rust-lang/rust/issues/54140>
            // derive_meta_list
            //     .span()
            //     .warning(
            //         "`GameObject` derive not necessary when the `game_object` attribute is used.",
            //     )
            //     .emit();
            panic!("`GameObject` derive not necessary when the `game_object` attribute is used.");
        } else {
            derive_meta_list
                .nested
                .push(NestedMeta::Meta(Meta::Word(Ident::new(
                    "GameObject",
                    Span::call_site(),
                ))));

            // Replace the existing `Attribute`.
            *attr = parse_quote!(#[#derive_meta_list]);
        }
    } else {
        // Add a new `#[derive(GameObject)]` attribute.
        let derive_attribute: Attribute = parse_quote!(#[derive(GameObject)]);
        ast.attrs.push(derive_attribute);
    }
}

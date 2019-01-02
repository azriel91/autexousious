use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, DeriveInput, Fields, FieldsNamed, Path};

use crate::{game_object_attribute_args::GameObjectAttributeArgs, util::data_struct_mut};

const ERR_MUST_BE_STRUCT: &str = "`game_object` attribute must be used on a struct.";
const ERR_MUST_BE_UNIT_OR_NAMED: &str =
    "`game_object` attribute must be used on either a unit struct or a struct with named fields.\n\
     This derive does not work on tuple structs.";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let data_struct = data_struct_mut(&mut ast, ERR_MUST_BE_STRUCT);

    object_fields_gen(&args.sequence_id, &mut data_struct.fields);

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
        pub object_handle: ObjectHandle<#sequence_id>,
        /// Component sequence transitions when a sequence ends.
        pub sequence_end_transitions: SequenceEndTransitions<#sequence_id>,
    })
}

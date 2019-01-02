use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, DeriveInput, Fields, FieldsNamed};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs,
    util::{data_struct_mut, ensure_fields_named_or_unit},
};

const ERR_MUST_BE_STRUCT: &str = "`game_object` attribute must be used on a struct.";
const ERR_MUST_BE_UNIT_OR_NAMED: &str =
    "`game_object` attribute must be used on either a unit struct or a struct with named fields.\n\
     This derive does not work on tuple structs.";

pub fn game_object_gen(args: GameObjectAttributeArgs, mut ast: DeriveInput) -> TokenStream {
    let mut data_struct = data_struct_mut(&mut ast, ERR_MUST_BE_STRUCT);
    let fields = &data_struct.fields;
    ensure_fields_named_or_unit(fields, ERR_MUST_BE_UNIT_OR_NAMED);

    let sequence_id = &args.sequence_id;
    let augmented_fields: FieldsNamed = parse_quote!({
        #(#fields,)*
        /// Handle to loaded object data.
        pub object_handle: ObjectHandle<#sequence_id>,
        /// Component sequence transitions when a sequence ends.
        pub sequence_end_transitions: SequenceEndTransitions<#sequence_id>,
    });

    data_struct.fields = Fields::from(augmented_fields);

    ast.into_token_stream().into()
}

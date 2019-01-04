use syn::{Data, DataStruct, DeriveInput};

/// Returns the `DataStruct` of this struct's `DeriveInput`.
///
/// # Parameters
///
/// * `ast`: `DeriveInput` representing the struct.
/// * `error_message`: Panic message if the `DeriveInput` is not for a struct.
///
/// # Panics
///
/// Panics if the `DeriveInput` is not for a `struct`.
pub fn data_struct_mut<'ast>(
    ast: &'ast mut DeriveInput,
    error_message: &'static str,
) -> &'ast mut DataStruct {
    match &mut ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!(error_message),
    }
}

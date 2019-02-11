use proc_macro2::Span;
use syn::{Data, DataStruct, DeriveInput, Ident};

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

/// Returns an `Ident` by concatenating `String` representations.
pub fn ident_concat(part_a: &str, part_b: &str) -> Ident {
    let mut combined = String::with_capacity(part_a.len() + part_b.len());
    combined.push_str(part_a);
    combined.push_str(part_b);

    Ident::new(&combined, Span::call_site())
}

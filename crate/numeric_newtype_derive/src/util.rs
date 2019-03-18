use syn::{
    Data, DataStruct, Field, Fields, Ident, Meta, MetaList, MetaNameValue, NestedMeta, Type,
    TypePath,
};

/// Returns the `Field` of an inner tuple struct.
///
/// # Parameters
///
/// * `data`: The newtype wrapper's data structure.
///
/// # Panics
///
/// Panics if the type is not a struct, or is a non-tuple struct, or has more than one tuple field.
pub fn inner_type(data: &Data) -> &Field {
    if let Data::Struct(DataStruct {
        fields: Fields::Unnamed(fields_unnamed),
        ..
    }) = data
    {
        if fields_unnamed.unnamed.len() == 1 {
            fields_unnamed
                .unnamed
                .first()
                .expect("Expected first field to exist.")
                .value()
        } else {
            panic!(
                "Newtype struct must only have one field.\n\
                 See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
                 for more information."
            )
        }
    } else {
        panic!(
            "`numeric_newtype` must be used on a newtype struct.\n\
             See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
             for more information."
        )
    }
}

/// Returns whether the `MetaList` contains a `Meta::Word` with the given ident.
///
/// This can be used to check if a `#[derive(..)]` contains `SomeDerive`.
///
/// # Parameters
///
/// * `meta_list`: The `MetaList` to check.
/// * `operand`: Ident that may be in the list.
pub fn meta_list_contains(meta_list: &MetaList, operand: &NestedMeta) -> bool {
    meta_list
        .nested
        .iter()
        .find(|nested_meta| nested_meta_eq(nested_meta, operand))
        .is_some()
}

fn nested_meta_eq(left: &NestedMeta, right: &NestedMeta) -> bool {
    match (left, right) {
        (NestedMeta::Meta(Meta::Word(left_ident)), NestedMeta::Meta(Meta::Word(right_ident))) => {
            left_ident == right_ident
        }
        _ => false,
    }
}

/// Returns the `Ident` of a nested meta. If it is a literal, `None` is returned.
///
/// # Parameters
///
/// * `nested_meta`: The `NestedMeta` to extract the `Ident` from.
pub fn nested_meta_to_ident(nested_meta: &NestedMeta) -> Option<&Ident> {
    match nested_meta {
        NestedMeta::Meta(meta) => Some(meta_to_ident(meta)),
        NestedMeta::Literal(..) => None,
    }
}

/// Returns the `Ident` of a `Meta` item.
///
/// # Parameters
///
/// * `meta`: The `Meta` item to extract the `Ident` from.
pub fn meta_to_ident(meta: &Meta) -> &Ident {
    match meta {
        Meta::Word(ident) => ident,
        Meta::List(MetaList { ident, .. }) => ident,
        Meta::NameValue(MetaNameValue { ident, .. }) => ident,
    }
}

/// Static list so we can do naive detection of types that `impl Eq + Ord`.
const KNOWN_TYPES_EQ_ORD: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
];

/// Returns whether the type is known to be `Eq + Ord`.
///
/// Note: This is a static list, so any types not in the following list will give a false `false`:
pub fn is_eq_ord(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        path.segments
            .last()
            .map(|segment| {
                let ty_ident = &segment.value().ident;
                KNOWN_TYPES_EQ_ORD
                    .iter()
                    .any(|known_ty| ty_ident == known_ty)
            })
            .unwrap_or(false)
    } else {
        false
    }
}

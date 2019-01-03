use syn::{
    punctuated::Pair, AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Field, Fields,
    GenericArgument, Meta, MetaList, NestedMeta, Path, PathArguments, Type, TypePath,
};

const IMPOSSIBLE_PATH_TRAILING_COLON2: &str = "`Path` cannot have a trailing `Colon2`.";

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

/// Returns the `Fields` of this struct's `DeriveInput`.
///
/// # Parameters
///
/// * `ast`: `DeriveInput` representing the struct.
/// * `error_message`: Panic message if the `DeriveInput` is not for a struct.
///
/// # Panics
///
/// Panics if the `DeriveInput` is not for a `struct`.
pub fn ast_fields<'ast>(ast: &'ast DeriveInput, error_message: &'static str) -> &'ast Fields {
    let fields = match &ast.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!(error_message),
    };

    fields
}

/// Panics if the fields are for a tuple or unit struct.
///
/// # Parameters
///
/// * `fields`: The fields to search through.
/// * `error_message`: Panic message if the `Fields` are not `Named`.
pub fn ensure_fields_named(fields: &Fields, error_message: &'static str) {
    match fields {
        Fields::Named(_) => {}
        Fields::Unnamed(_) | Fields::Unit => panic!(error_message),
    };
}

/// Returns whether the `MetaList` contains a `Meta::Word` with the given ident.
///
/// This can be used to check if a `#[derive(..)]` contains `SomeDerive`.
///
/// # Parameters
///
/// * `meta_list`: The `MetaList` to check.
/// * `ident`: Ident that may be in the list.
pub fn meta_list_contains(meta_list: &MetaList, ident: &str) -> bool {
    meta_list
        .nested
        .iter()
        .find(|nested_meta| {
            if let NestedMeta::Meta(Meta::Word(word)) = nested_meta {
                word == ident
            } else {
                false
            }
        })
        .is_some()
}

/// Returns the `Field` whose top level type name matches the search string.
///
/// Note: This returns the first field with the given type, and does not attempt to detect and fail
/// when multiple fields have that type.
///
/// # Parameters
///
/// * `fields`: The fields to search through.
/// * `ty`: Last segment of the path of the type to look for, e.g. "ObjectHandle"
pub fn find_field<'f>(fields: &'f Fields, ty: &str) -> Option<&'f Field> {
    fields.iter().find(|f| match &f.ty {
        Type::Path(ty_path) => {
            if let Some(Pair::End(segment)) = ty_path.path.segments.last() {
                segment.ident == ty
            } else {
                unreachable!(IMPOSSIBLE_PATH_TRAILING_COLON2)
            }
        }
        _ => false,
    })
}

/// Returns the first path type parameter if it exists.
///
/// # Parameters
///
/// * `ty`: Type to look for a type parameter, e.g. `MyType<Param>`.
pub fn first_type_param(ty: &Type) -> Option<&Path> {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            if let Some(Pair::End(segment)) = segments.last() {
                match &segment.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args, ..
                    }) => args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(Type::Path(TypePath { path, .. })) => Some(path),
                        _ => None,
                    }),
                    _ => None,
                }
            } else {
                unreachable!(IMPOSSIBLE_PATH_TRAILING_COLON2)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    macro_rules! fields_named {
        ($($fields:tt)*) => {
            {
                use quote::quote;
                use syn::{parse_quote, FieldsNamed};

                let fields_named : FieldsNamed = parse_quote! { { $($fields)* } };
                fields_named
            }
        }
    }

    macro_rules! fields_unnamed {
        ($($fields:tt)*) => {
            {
                use quote::quote;
                use syn::{parse_quote, FieldsUnnamed};

                let fields_unnamed : FieldsUnnamed = parse_quote! { ($($fields)*,) };
                fields_unnamed
            }
        }
    }

    mod ensure_fields_named {
        use quote::quote;
        use syn::{parse_quote, Fields};

        use crate::util::ensure_fields_named;

        #[test]
        fn no_op_when_struct_fields_are_named() {
            let fields = fields_named! { f: AType }.into();

            ensure_fields_named(&fields, "rara");
        }

        #[test]
        #[should_panic(expected = "rara")]
        fn panics_when_struct_fields_are_unnamed() {
            let fields = fields_unnamed! { AType }.into();

            ensure_fields_named(&fields, "rara");
        }

        #[test]
        #[should_panic(expected = "rara")]
        fn panics_when_struct_is_unit() {
            let fields = Fields::Unit;

            ensure_fields_named(&fields, "rara");
        }
    }

    mod find_field {
        use crate::util::find_field;

        macro_rules! data_struct {
            ($($fields:tt)*) => {
                {
                    use quote::quote;
                    use syn::{parse_quote, Data, DeriveInput};

                    let ast : DeriveInput = parse_quote! { struct S { $($fields)* } };
                    match ast.data {
                        Data::Struct(data_struct) => data_struct,
                        _ => unreachable!(),
                    }
                }
            }
        }

        macro_rules! field {
            ($($fields:tt)*) => {
                {
                    use syn::{punctuated::Pair};

                    let fields_named = fields_named! { $($fields)* };
                    let pair = fields_named.named
                        .first()
                        .expect("At least one field must be passed in.");

                    match pair {
                        Pair::Punctuated(field, _) | Pair::End(field) => field.clone(),
                    }
                }
            }
        }

        #[test]
        fn returns_none_when_ty_is_not_path() {
            let data_struct = data_struct! { f: (u8,) };

            assert_eq!(None, find_field(&data_struct.fields, "Unused"));
        }

        #[test]
        fn returns_none_when_ty_does_not_match() {
            let data_struct = data_struct! { f: AType };

            assert_eq!(None, find_field(&data_struct.fields, "DifferentType"));
        }

        #[test]
        fn returns_ty_with_matching_name() {
            let data_struct = data_struct! { f: AType, f2: BType<TypeParam> };

            let expected = field! { f2: BType<TypeParam> };
            assert_eq!(Some(&expected), find_field(&data_struct.fields, "BType"));
        }
    }

    mod first_type_param {
        use quote::quote;
        use syn::parse_quote;

        use crate::util::first_type_param;

        #[test]
        fn returns_none_when_ty_is_not_path() {
            let ty = parse_quote! { _ };

            assert_eq!(None, first_type_param(&ty));
        }

        #[test]
        fn returns_none_when_ty_has_no_params() {
            let ty = parse_quote! { NoParamType };

            assert_eq!(None, first_type_param(&ty));
        }

        #[test]
        fn returns_none_when_ty_param_is_lifetime() {
            let ty = parse_quote! { OnlyLifetime<'life> };

            assert_eq!(None, first_type_param(&ty));
        }

        #[test]
        fn returns_type_when_ty_param_is_path() {
            let ty = parse_quote! { OneParam<TypeParam> };

            let expected = parse_quote! { TypeParam };

            assert_eq!(Some(&expected), first_type_param(&ty));
        }

        #[test]
        fn returns_first_type_when_multiple_type_params_exist() {
            let ty = parse_quote! { MultiParam<TypeParam, Param2, Param3, Param4, Param5> };

            let expected = parse_quote! { TypeParam };
            assert_eq!(Some(&expected), first_type_param(&ty));
        }

        #[test]
        fn returns_first_type_when_lifetimes_and_type_params_exist() {
            let ty = parse_quote! { MultiParam<'one, 'two, TypeParam, Param2, Param3> };

            let expected = parse_quote! { TypeParam };
            assert_eq!(Some(&expected), first_type_param(&ty));
        }
    }
}

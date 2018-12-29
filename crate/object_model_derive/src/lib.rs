//! Provides the `#[derive(GameObject)]` proc_macro to implement the `GameObject` trait.
//!
//! For example usage of this macro, refer to the documentation for the `GameObject` trait.

extern crate proc_macro;

use quote::quote;
use syn::{
    punctuated::Pair, AngleBracketedGenericArguments, Data, DeriveInput, Field, Fields,
    GenericArgument, Path, PathArguments, Type, TypePath,
};

use proc_macro::TokenStream;

const IMPOSSIBLE_PATH_TRAILING_COLON2: &str = "`Path` cannot have a trailing `Colon2`.";

/// Derives `GameObject` on a struct.
#[proc_macro_derive(GameObject)]
pub fn object_type(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input)
        .unwrap_or_else(|e| panic!("Failed to parse token stream. Error: {}", e));

    impl_object_type(&ast)
}

fn impl_object_type(ast: &syn::DeriveInput) -> TokenStream {
    let ty_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("`GameObject` derive must be used on a struct."),
    };
    ensure_struct_fields_are_named(fields);

    let object_handle_field = find_field(fields, stringify!(ObjectHandle))
        .unwrap_or_else(|| panic!("Unable to find field with type: `ObjectHandle<_>`."));
    let object_handle_field_name = &object_handle_field.ident;
    let sequence_end_transitions_field = find_field(fields, stringify!(SequenceEndTransitions))
        .unwrap_or_else(|| panic!("Unable to find field with type: `SequenceEndTransitions<_>`."));
    let sequence_end_transitions_field_name = &sequence_end_transitions_field.ident;

    let sequence_id_type = {
        let handle_seq_id_type = first_type_param(&object_handle_field.ty).unwrap_or_else(|| {
            panic!(
                "Failed to read first type parameter of `{}` field.",
                object_handle_field_name.as_ref().unwrap()
            )
        });

        let transitions_seq_id_type = first_type_param(&sequence_end_transitions_field.ty)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to read first type parameter of `{}` field.",
                    sequence_end_transitions_field_name.as_ref().unwrap()
                )
            });

        if handle_seq_id_type != transitions_seq_id_type {
            panic!(
                "`ObjectHandle` sequence ID type: `{:?}` \
                 must match `SequenceEndTransitions` sequence ID type: `{:?}`",
                handle_seq_id_type, transitions_seq_id_type
            );
        }

        handle_seq_id_type
    };

    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let gen = quote! {
        impl #impl_generics GameObject<#sequence_id_type> for #ty_name #ty_generics #where_clause {
            fn object_handle(&self) -> &ObjectHandle<#sequence_id_type> {
                &self.#object_handle_field_name
            }

            fn sequence_end_transitions(&self) -> &SequenceEndTransitions<#sequence_id_type> {
                &self.#sequence_end_transitions_field_name
            }
        }
    };
    gen.into()
}

/// Panics if the fields are for a tuple or unit struct.
fn ensure_struct_fields_are_named(fields: &Fields) {
    match fields {
        Fields::Named(_) => {}
        Fields::Unnamed(_) | Fields::Unit => panic!(
            "`GameObject` derive must be used on a struct with named fields.\n\
             This derive does not work on tuple or unit structs."
        ),
    };
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
fn find_field<'f>(fields: &'f Fields, ty: &str) -> Option<&'f Field> {
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
fn first_type_param(ty: &Type) -> Option<&Path> {
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

    mod ensure_struct_fields_are_named {
        use quote::quote;
        use syn::{parse_quote, Fields};

        use crate::ensure_struct_fields_are_named;

        #[test]
        fn no_op_when_struct_fields_are_named() {
            let fields = fields_named! { f: AType }.into();

            ensure_struct_fields_are_named(&fields);
        }

        #[test]
        #[should_panic(
            expected = "`GameObject` derive must be used on a struct with named fields.\n\
                        This derive does not work on tuple or unit structs."
        )]
        fn panics_when_struct_fields_are_unnamed() {
            let fields = fields_unnamed! { AType }.into();

            ensure_struct_fields_are_named(&fields);
        }

        #[test]
        #[should_panic(
            expected = "`GameObject` derive must be used on a struct with named fields.\n\
                        This derive does not work on tuple or unit structs."
        )]
        fn panics_when_struct_is_unit() {
            let fields = Fields::Unit;

            ensure_struct_fields_are_named(&fields);
        }
    }

    mod find_field {
        use crate::find_field;

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

        use crate::first_type_param;

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

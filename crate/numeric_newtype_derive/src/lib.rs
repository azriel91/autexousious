#![recursion_limit = "128"]

//! Provides the `#[numeric_newtype]` proc_macro_attribute to implement
//! `std::ops::*` traits.
//!
//! # Examples
//!
//! ```edition2018
//! use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
//! use numeric_newtype_derive::numeric_newtype;
//!
//! macro_rules! test {
//!     ($name:ident, $ty:ident, $v0:expr, $v1:expr, $zero:expr) => {
//!         /// The newtype.
//!         #[numeric_newtype]
//!         #[derive(Debug)]
//!         struct $name($ty);
//!
//!         let plain: $ty = $v0;
//!         let plain_small: $ty = $v1;
//!
//!         // Check `From::from` and `$name::new`.
//!         let points = $name::from(plain);
//!         let points_small = $name::new(plain_small);
//!
//!         // std::ops::_ + PartialEq
//!         assert_eq!(points, plain);
//!         assert_eq!(points + points, $v0 + $v0);
//!         assert_eq!(points + plain, $v0 + $v0);
//!         assert_eq!(points - points, $zero);
//!         assert_eq!(points - plain, $zero);
//!
//!         // PartialOrd
//!         assert!(points > points_small);
//!         assert!(points > plain_small);
//!     };
//! }
//!
//! test!(PointsU8, u8, 10, 9, 0);
//! test!(PointsU16, u16, 10, 9, 0);
//! test!(PointsU32, u32, 10, 9, 0);
//! test!(PointsU64, u64, 10, 9, 0);
//! test!(PointsU128, u128, 10, 9, 0);
//! test!(PointsI8, i8, 10, 9, 0);
//! test!(PointsI16, i16, 10, 9, 0);
//! test!(PointsI32, i32, 10, 9, 0);
//! test!(PointsI64, i64, 10, 9, 0);
//! test!(PointsI128, i128, 10, 9, 0);
//! test!(PointsF32, f32, 10., 9., 0.);
//! test!(PointsF64, f64, 10., 9., 0.);
//! ```

// TODO: Test using `compiletest_rs`.
//
// At the time of writing, we cannot test using `compiletest_rs` as it does not
// appear to be able to link to external crates, so it never resolves
// `derive_more` as a dependency.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputExt, DeriveInputNewtypeExt};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, DeriveInput, NestedMeta,
    Type, TypePath,
};

/// Static list so we can do naive detection of types that `impl Eq + Ord`.
const KNOWN_TYPES_EQ_ORD: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
];

#[proc_macro_attribute]
pub fn numeric_newtype(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);

    let is_eq_ord = {
        let inner_type = ast.inner_type();
        let inner_type = &inner_type.ty;
        is_eq_ord(inner_type)
    };

    derive_gen(&mut ast, is_eq_ord);
    let type_name = &ast.ident;
    let inner_type = ast.inner_type();
    let inner_type = &inner_type.ty;

    let doc_fn_new = format!("Returns a new {}.", type_name);
    let token_stream_2 = quote! {
        #ast

        impl #type_name {
            #[doc = #doc_fn_new]
            pub fn new(value: #inner_type) -> Self {
                #type_name(value)
            }
        }

        impl std::ops::Deref for #type_name {
            type Target = #inner_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #type_name {
            fn deref_mut(&mut self) -> &mut #inner_type {
                &mut self.0
            }
        }

        impl std::ops::Add<#inner_type> for #type_name {
            type Output = Self;

            fn add(self, other: #inner_type) -> Self {
                #type_name(self.0 + other)
            }
        }

        impl std::ops::AddAssign<#inner_type> for #type_name {
            fn add_assign(&mut self, other: #inner_type) {
                *self = #type_name(self.0 + other);
            }
        }

        impl std::ops::Sub<#inner_type> for #type_name {
            type Output = Self;

            fn sub(self, other: #inner_type) -> Self {
                #type_name(self.0 - other)
            }
        }

        impl std::ops::SubAssign<#inner_type> for #type_name {
            fn sub_assign(&mut self, other: #inner_type) {
                *self = #type_name(self.0 - other);
            }
        }

        impl std::cmp::PartialOrd<#inner_type> for #type_name {
            fn partial_cmp(&self, other: &#inner_type) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl std::cmp::PartialEq<#inner_type> for #type_name {
            fn eq(&self, other: &#inner_type) -> bool {
                self.0 == *other
            }
        }
    };
    TokenStream::from(token_stream_2)
}

/// Adds numeric derives to the list of derives.
///
/// These are:
///
/// * `derive_more::Add`
/// * `derive_more::AddAssign`
/// * `derive_more::Sub`
/// * `derive_more::SubAssign`
/// * `derive_more::Display`
/// * `derive_more::From`
/// * `Clone`
/// * `Copy`
/// * `PartialEq`
/// * `PartialOrd`
///
/// Based on a static list, if the inner type derives `Eq + Ord`, those will
/// also be derived.
fn derive_gen(ast: &mut DeriveInput, derive_eq_ord: bool) {
    let derives = {
        let mut base_derives: Punctuated<NestedMeta, Comma> = parse_quote!(
            Add, AddAssign, Sub, SubAssign, Display, From, Clone, Copy, PartialEq, PartialOrd
        );
        if derive_eq_ord {
            let additional_derives: Punctuated<NestedMeta, Comma> = parse_quote!(Eq, Ord);
            base_derives.extend(additional_derives);
        }

        base_derives
    };

    ast.append_derives(derives);
}

/// Returns whether the type is known to be `Eq + Ord`.
///
/// Note: This is a static list, so any types not in the following list will
/// give a false `false`:
fn is_eq_ord(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        path.segments
            .last()
            .map(|segment| {
                let ty_ident = &segment.ident;
                KNOWN_TYPES_EQ_ORD
                    .iter()
                    .any(|known_ty| ty_ident == known_ty)
            })
            .unwrap_or(false)
    } else {
        false
    }
}

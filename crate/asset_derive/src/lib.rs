#![recursion_limit = "128"]

//! Provides the `#[derive(Asset)]` macro to implement the `amethyst::assets::Asset` trait.
//!
//! # Examples
//!
//! ```rust,edition2018
//! use asset_derive::Asset;
//!
//! #[derive(Asset, Clone, Debug)]
//! pub struct MyAsset {
//!     pub value: u32,
//! }
//! ```
//!
//! Effectively generates the following without introducing the imports into scope:
//!
//! ```rust,ignore
//! use amethyst::{
//!     assets::{Asset, Handle, ProcessingState},
//!     ecs::storage::VecStorage,
//!     Error,
//! }
//!
//! impl Asset for MyAsset {
//!     type Data = Self;
//!     type HandleStorage = VecStorage<Handle<Self>>;
//!
//!     const NAME: &'static str = concat!(module_path!(), "::", stringify!(MyAsset));
//! }
//!
//! impl From<MyAsset> for Result<ProcessingState<MyAsset>, Error> {
//!     fn from(asset_data: MyAsset) -> Result<ProcessingState<MyAsset>, Error> {
//!         Ok(ProcessingState::Loaded(asset_data))
//!     }
//! }
//! ```
//!
//! # Errors
//!
//! If the asset type is type parameterized (`MyAsset<T>`), you may encounter the following error:
//!
//! ```ignore
//! error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
//!   --> crate_name\src\my_asset.rs:11:10
//!    |
//! 11 | #[derive(Asset, Clone, Debug)]
//!    |          ^^^^^ type parameter `T` must be used as the type parameter for some local type
//!    |
//!    = note: only traits defined in the current crate can be implemented for a type parameter
//! ```
//!
//! This means `T` may be provided by a downstream crate, meaning `MyAsset<T>` is defined by that
//! downstream crate. Therefore, implementing `Asset` breaks the orphan rule.
//!
//! To work past this rule, implement a newtype wrapper instead.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::IdentExt;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Meta};

const ASSET_DATA_ATTRIBUTE: &str = "asset_data";

#[proc_macro_derive(Asset, attributes(asset_data))]
pub fn derive_asset(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let type_name = &ast.ident;
    let asset_data = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident(ASSET_DATA_ATTRIBUTE))
        .map(|attr| {
            attr.parse_meta().unwrap_or_else(|e| {
                panic!(
                    "Failed to parse `{}` attribute as meta. Error: {}",
                    ASSET_DATA_ATTRIBUTE, e
                )
            })
        })
        .map(|meta| {
            if let Meta::Path(path) = meta {
                path
            } else {
                panic!(
                    "Expected type name in `{}` attribute.",
                    ASSET_DATA_ATTRIBUTE
                )
            }
        })
        .unwrap_or_else(|| parse_quote!(Self));
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let type_name_string = type_name.to_string();
    let preposition = if begins_with_vowel(&type_name_string) {
        "an"
    } else {
        "a"
    };
    let handle_type_name = type_name.append("Handle");
    let handle_doc = format!("Handle to {} `{}`.", preposition, type_name_string);

    let mut token_stream_2 = quote! {
        impl #impl_generics amethyst::assets::Asset for #type_name #ty_generics #where_clause {
            type Data = #asset_data;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;

            const NAME: &'static str =
                concat!(module_path!(), "::", stringify!(#type_name), stringify!(#ty_generics));
        }

        #[doc = #handle_doc]
        pub type #handle_type_name #ty_generics = amethyst::assets::Handle<#type_name #ty_generics>;
    };

    // Generate trivial processor impl for `Self` asset datas.
    if asset_data == parse_quote!(Self) {
        token_stream_2.extend(quote! {
            impl #impl_generics std::convert::From<#type_name #ty_generics> for
                std::result::Result<
                    amethyst::assets::ProcessingState<#type_name #ty_generics>,
                    amethyst::Error
                >
            #where_clause
            {
                fn from(
                    asset_data: #type_name #ty_generics,
                )
                -> std::result::Result<
                    amethyst::assets::ProcessingState<#type_name #ty_generics>,
                    amethyst::Error
                >
                {
                    Ok(amethyst::assets::ProcessingState::Loaded(asset_data))
                }
            }
        });
    }

    TokenStream::from(token_stream_2)
}

fn begins_with_vowel(word: &str) -> bool {
    if let Some('A') | Some('E') | Some('I') | Some('O') | Some('U') = word.chars().next() {
        true
    } else {
        false
    }
}

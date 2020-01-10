#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a proc macro derive and trait to produce a struct from enum variants.
//!
//! This is a poor-man's implementation of <https://github.com/rust-lang/rfcs/pull/2593>.
//!
//! # Examples
//!
//! ```rust,edition2018
//! use variant_struct::VariantStruct;
//!
//! #[derive(VariantStruct)]
//! pub enum MyEnum {
//!     /// Unit variant.
//!     Unit,
//!     /// Tuple variant.
//!     Tuple(u32, u64),
//!     /// Struct variant.
//!     Struct {
//!         field_0: u32,
//!         field_1: u64,
//!     },
//! }
//! ```
//!
//! Generates:
//!
//! ```rust,edition2018
//! use std::convert::TryFrom;
//!
//! use variant_struct::VariantStruct;
//!
//! # pub enum MyEnum {
//! #     /// Unit variant.
//! #     Unit,
//! #     /// Tuple variant.
//! #     Tuple(u32, u64),
//! #     /// Struct variant.
//! #     Struct {
//! #         field_0: u32,
//! #         field_1: u64,
//! #     },
//! # }
//! #
//! /// Unit variant.
//! #[derive(Debug)]
//! pub struct Unit;
//!
//! /// Tuple variant.
//! #[derive(Debug)]
//! pub struct Tuple(pub u32, pub u64);
//!
//! /// Struct variant.
//! #[derive(Debug)]
//! pub struct Struct {
//!     pub field_0: u32,
//!     pub field_1: u64,
//! }
//!
//! impl From<Unit> for MyEnum {
//!     fn from(variant_struct: Unit) -> Self {
//!         MyEnum::Unit
//!     }
//! }
//!
//! impl From<Tuple> for MyEnum {
//!     fn from(variant_struct: Tuple) -> Self {
//!         let Tuple(_0, _1) = variant_struct;
//!         MyEnum::Tuple(_0, _1)
//!     }
//! }
//!
//! impl From<Struct> for MyEnum {
//!     fn from(variant_struct: Struct) -> Self {
//!         let Struct { field_0, field_1 } = variant_struct;
//!         MyEnum::Struct { field_0, field_1 }
//!     }
//! }
//!
//! impl TryFrom<MyEnum> for Unit {
//!     type Error = MyEnum;
//!     fn try_from(enum_variant: MyEnum) -> Result<Self, Self::Error> {
//!         if let MyEnum::Unit = enum_variant {
//!             Ok(Unit)
//!         } else {
//!             Err(enum_variant)
//!         }
//!     }
//! }
//!
//! impl TryFrom<MyEnum> for Tuple {
//!     type Error = MyEnum;
//!     fn try_from(enum_variant: MyEnum) -> Result<Self, Self::Error> {
//!         if let MyEnum::Tuple(_0, _1) = enum_variant {
//!             Ok(Tuple(_0, _1))
//!         } else {
//!             Err(enum_variant)
//!         }
//!     }
//! }
//!
//! impl TryFrom<MyEnum> for Struct {
//!     type Error = MyEnum;
//!     fn try_from(enum_variant: MyEnum) -> Result<Self, Self::Error> {
//!         if let MyEnum::Struct { field_0, field_1 } = enum_variant {
//!             Ok(Struct { field_0, field_1 })
//!         } else {
//!             Err(enum_variant)
//!         }
//!     }
//! }
//! ```

pub use variant_struct_derive::VariantStruct;

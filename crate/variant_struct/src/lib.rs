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
//!         field: u32,
//!     },
//! }
//! ```
//!
//! Generates:
//!
//! ```rust,edition2018
//! use variant_struct::VariantStruct;
//!
//! # pub enum MyEnum {
//! #     /// Unit variant.
//! #     Unit,
//! #     /// Tuple variant.
//! #     Tuple(u32, u64),
//! #     /// Struct variant.
//! #     Struct {
//! #         field: u32,
//! #     },
//! # }
//! #
//! /// Unit variant.
//! #[derive(Debug)]
//! pub struct Unit;
//! /// Tuple variant.
//! #[derive(Debug)]
//! pub struct Tuple(pub u32, pub u64);
//! /// Struct variant.
//! #[derive(Debug)]
//! pub struct Struct {
//!     pub field: u32,
//! }
//!
//! impl variant_struct::OfVariant for Unit {
//!     type Enum = MyEnum;
//!
//!     fn is_for(value: &MyEnum) -> bool {
//!         if let MyEnum::Unit = value {
//!             true
//!         } else {
//!             false
//!         }
//!     }
//! }
//!
//! impl variant_struct::OfVariant for Tuple {
//!     type Enum = MyEnum;
//!
//!     fn is_for(value: &MyEnum) -> bool {
//!         if let MyEnum::Tuple(..) = value {
//!             true
//!         } else {
//!             false
//!         }
//!     }
//! }
//!
//! impl variant_struct::OfVariant for Struct {
//!     type Enum = MyEnum;
//!
//!     fn is_for(value: &MyEnum) -> bool {
//!         if let MyEnum::Struct { .. } = value {
//!             true
//!         } else {
//!             false
//!         }
//!     }
//! }
//! ```

pub use variant_struct_derive::VariantStruct;

/// Indicates the struct corresponding to an enum variant.
///
/// This trait is not actually useful for anything right now.
pub trait VariantStruct {
    /// The struct corresponding to this variant.
    type Struct;
}

/// Indicates the struct corresponding to an enum variant.
///
/// This is a placeholder while <https://github.com/rust-lang/rfcs/pull/2593>
pub trait OfVariant {
    /// The enum type this struct belongs to.
    type Enum;
    /// Returns whether this `struct` is for
    fn is_for(value: &Self::Enum) -> bool;
}

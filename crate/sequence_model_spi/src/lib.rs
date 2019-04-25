#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Service provider interface library for crates that extend the `object_model`.
//!
//! For example, the [`ComponentFrames<C>`] type from this crate is used to parameterize frame
//! components such as `Body`.
//!
//! [`ComponentFrames<C>`]: loaded/struct.ComponentFrames.html

pub mod loaded;

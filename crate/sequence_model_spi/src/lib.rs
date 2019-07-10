#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Service provider interface library for crates that extend the `sequence_model`.
//!
//! For example, the [`ComponentSequence<C>`] type from this crate is used to parameterize frame
//! components such as `Body`.
//!
//! [`ComponentSequence<C>`]: loaded/struct.ComponentSequence.html

pub mod loaded;

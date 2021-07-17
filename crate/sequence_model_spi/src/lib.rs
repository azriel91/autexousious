#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Service provider interface library for crates that extend the
//! `sequence_model`.
//!
//! For example, the [`FrameComponentData<C>`] type from this crate is used to
//! parameterize frame components such as `Body`.
//!
//! [`FrameComponentData<C>`]: loaded/struct.FrameComponentData.html

pub mod loaded;

#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to manage logic clocks.
//!
//! # Examples
//!
//! Deriving a logic clock newtype.
//!
//! ```rust,edition2018
//! use amethyst::ecs::{storage::DenseVecStorage, Component};
//! use derive_deref::{Deref, DerefMut};
//! use derive_more::From;
//! use logic_clock::logic_clock;
//! use serde::{Deserialize, Serialize};
//!
//! /// Logic clock to track frame index of an object sequence.
//! #[logic_clock]
//! pub struct FrameIndexClock;
//! ```

pub use logic_clock_derive::logic_clock;

pub use crate::logic_clock::LogicClock;

mod logic_clock;

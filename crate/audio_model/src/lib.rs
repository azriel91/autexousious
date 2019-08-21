#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent audio -- background music (BGM) and sound effects (SFX).
//!
//! There are two overarching ways that determine when to play a sound:
//!
//! * **Application:** Application logic intends to play a sound.
//! * **Configuration:** Object configuration specified sounds.
//!
//! ## Application
//!
//! Application encoded sounds are known at development time. Examples include:
//!
//! * Play a sound when a menu item is selected.
//! * Play a sound when an `Interaction` of this kind hits a `Body` of this kind.
//!
//! The actual sound to play is still configuration; the logical definition that the "menu_selected"
//! sound should be played is what makes it an application encoded sound. Since they are know, they
//! can be referred to using logical IDs, such as enum variants.
//!
//! ## Configuration
//!
//! Configuration encoded sounds are defined in object configuration -- `object.yaml`. These are not
//! known at application development time, so there is no known value to refer to them.
//!
//! In this case, we can use a `HashMap<String, AudioData>` (with better names), so that paths to
//! any audio file can be specified in object configuration.

pub mod config;
pub mod loaded;

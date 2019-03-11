#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Heads-up-display (HUD) types to provide information in game.

pub use crate::{hp_bar::HpBar, prefab::HpBarPrefab};

mod hp_bar;
mod prefab;

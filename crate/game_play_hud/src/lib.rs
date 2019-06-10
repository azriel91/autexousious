#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Heads-up-display (HUD) types to provide information in game.

pub use crate::{
    constants::{HP_BAR_HEIGHT, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT},
    hp_bar::HpBar,
    prefab::HpBarPrefab,
    system::HpBarUpdateSystem,
};

mod constants;
mod hp_bar;
mod prefab;
mod system;

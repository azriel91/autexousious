#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Heads-up-display (HUD) types to provide information in game.

pub use crate::{
    constants::{
        CP_BAR_HEIGHT, CP_BAR_LENGTH, CP_BAR_SPRITE_COUNT, HP_BAR_HEIGHT, HP_BAR_LENGTH,
        HP_BAR_SPRITE_COUNT,
    },
    cp_bar::CpBar,
    hp_bar::HpBar,
    prefab::{CpBarPrefab, HpBarPrefab},
    system::{CpBarUpdateSystem, HpBarUpdateSystem},
};

mod constants;
mod cp_bar;
mod hp_bar;
mod prefab;
mod system;

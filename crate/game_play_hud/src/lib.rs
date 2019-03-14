#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Heads-up-display (HUD) types to provide information in game.

pub use crate::{
    constants::{HP_BAR_HEIGHT, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT},
    game_play_hud_bundle::GamePlayHudBundle,
    hp_bar::HpBar,
    prefab::HpBarPrefab,
    system::HpBarUpdateSystem,
};

mod constants;
mod game_play_hud_bundle;
mod hp_bar;
mod prefab;
mod system;

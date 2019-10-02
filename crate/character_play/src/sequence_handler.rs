//! Sequence transition handling for character sequence names.

use character_model::config::CharacterSequenceName;

use crate::CharacterSequenceUpdateComponents;

pub use self::{
    dash_attack::DashAttack, dash_back::DashBack, dash_back_ascend::DashBackAscend,
    dash_back_descend::DashBackDescend, dash_descend_land::DashDescendLand,
    dash_forward::DashForward, dash_forward_ascend::DashForwardAscend,
    dash_forward_descend::DashForwardDescend, dodge::Dodge, fall_forward_ascend::FallForwardAscend,
    fall_forward_descend::FallForwardDescend, fall_forward_land::FallForwardLand, jump::Jump,
    jump_ascend::JumpAscend, jump_attack::JumpAttack, jump_descend::JumpDescend,
    jump_descend_land::JumpDescendLand, jump_off::JumpOff, lie_face_down::LieFaceDown, run::Run,
    run_stop::RunStop, sequence_handler_util::SequenceHandlerUtil, stand::Stand,
    stand_attack::StandAttack, stand_on_sequence_end::StandOnSequenceEnd,
    switch_sequence_on_descend::SwitchSequenceOnDescend,
    switch_sequence_on_end::SwitchSequenceOnEnd,
    switch_sequence_on_end_y_velocity::SwitchSequenceOnEndYVelocity,
    switch_sequence_on_land::SwitchSequenceOnLand, walk::Walk,
};

pub mod common;

mod dash_attack;
mod dash_back;
mod dash_back_ascend;
mod dash_back_descend;
mod dash_descend_land;
mod dash_forward;
mod dash_forward_ascend;
mod dash_forward_descend;
mod dodge;
mod fall_forward_ascend;
mod fall_forward_descend;
mod fall_forward_land;
mod jump;
mod jump_ascend;
mod jump_attack;
mod jump_descend;
mod jump_descend_land;
mod jump_off;
mod lie_face_down;
mod run;
mod run_stop;
mod sequence_handler_util;
mod stand;
mod stand_attack;
mod stand_on_sequence_end;
mod switch_sequence_on_descend;
mod switch_sequence_on_end;
mod switch_sequence_on_end_y_velocity;
mod switch_sequence_on_land;
mod walk;

/// Sequence transition behaviour calculation.
pub trait CharacterSequenceHandler {
    /// Returns the status update for a character based on current input or lack thereof.
    ///
    /// Returns `Some(..)` when there is an update, `None` otherwise.
    ///
    /// # Parameters
    ///
    /// * `components`: Components used to compute character sequence updates.
    fn update(_components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        None
    }
}

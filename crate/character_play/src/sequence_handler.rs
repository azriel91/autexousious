use character_model::config::CharacterSequenceName;

use crate::CharacterSequenceUpdateComponents;

pub(crate) use self::sequence_handler_util::SequenceHandlerUtil;
pub(super) use self::{
    dash_attack::DashAttack, dash_back::DashBack, dash_back_ascend::DashBackAscend,
    dash_back_descend::DashBackDescend, dash_descend_land::DashDescendLand,
    dash_forward::DashForward, dash_forward_ascend::DashForwardAscend,
    dash_forward_descend::DashForwardDescend, dodge::Dodge, fall_forward_ascend::FallForwardAscend,
    fall_forward_descend::FallForwardDescend, fall_forward_land::FallForwardLand, jump::Jump,
    jump_ascend::JumpAscend, jump_attack::JumpAttack, jump_descend::JumpDescend,
    jump_descend_land::JumpDescendLand, jump_off::JumpOff, lie_face_down::LieFaceDown, run::Run,
    run_stop::RunStop, stand::Stand, stand_attack::StandAttack,
    stand_on_sequence_end::StandOnSequenceEnd, switch_sequence_on_descend::SwitchSequenceOnDescend,
    switch_sequence_on_end::SwitchSequenceOnEnd,
    switch_sequence_on_end_y_velocity::SwitchSequenceOnEndYVelocity,
    switch_sequence_on_land::SwitchSequenceOnLand, walk::Walk,
};

pub(super) mod common;
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
pub(super) trait CharacterSequenceHandler {
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

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::CharacterSequenceHandler;
    use crate::CharacterSequenceUpdateComponents;

    #[test]
    fn sequence_handler_default_update_is_none() {
        assert_eq!(
            None,
            Sit::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                &CharacterSequenceName::default(),
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    struct Sit;
    impl CharacterSequenceHandler for Sit {}
}

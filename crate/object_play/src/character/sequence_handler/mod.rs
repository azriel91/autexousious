use object_model::config::object::CharacterSequenceId;

use CharacterSequenceUpdateComponents;

pub(super) use self::fall_forward_ascend::FallForwardAscend;
pub(super) use self::fall_forward_descend::FallForwardDescend;
pub(super) use self::fall_forward_land::FallForwardLand;
pub(super) use self::jump::Jump;
pub(super) use self::jump_ascend::JumpAscend;
pub(super) use self::jump_descend::JumpDescend;
pub(super) use self::jump_descend_land::JumpDescendLand;
pub(super) use self::jump_off::JumpOff;
pub(super) use self::lie_face_down::LieFaceDown;
pub(super) use self::run::Run;
pub(super) use self::run_stop::RunStop;
pub(crate) use self::sequence_handler_util::SequenceHandlerUtil;
pub(super) use self::stand::Stand;
pub(super) use self::stand_attack::StandAttack;
pub(super) use self::stand_on_sequence_end::StandOnSequenceEnd;
pub(super) use self::switch_sequence_on_descend::SwitchSequenceOnDescend;
pub(super) use self::switch_sequence_on_end::SwitchSequenceOnEnd;
pub(super) use self::switch_sequence_on_land::SwitchSequenceOnLand;
pub(super) use self::walk::Walk;

pub(super) mod common;
mod fall_forward_ascend;
mod fall_forward_descend;
mod fall_forward_land;
mod jump;
mod jump_ascend;
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
    fn update<'c>(
        _components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        None
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn sequence_handler_default_update_is_none() {
        assert_eq!(
            None,
            Sit::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &CharacterSequenceId::default(),
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored::default(),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    struct Sit;
    impl CharacterSequenceHandler for Sit {}
}

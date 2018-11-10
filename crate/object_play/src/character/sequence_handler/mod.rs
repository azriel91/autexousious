use game_input::ControllerInput;
use object_model::entity::{CharacterStatus, CharacterStatusUpdate, Kinematics};

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
pub(super) use self::sequence_handler_util::SequenceHandlerUtil;
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
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `kinematics`: Kinematics of the character.
    fn update(
        _controller_input: &ControllerInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        CharacterStatusUpdate::default()
    }
}

/// Sequence transition behaviour calculation.
///
/// This serves the same purpose as `CharacterSequenceHandler`, except it allows for chaining
/// multiple calls together, useful for linking multiple common sequence handler logic blocks.
pub(super) trait SequenceHandler {
    /// Returns the status update for a character based on current input or lack thereof.
    ///
    /// Returns `Some(..)` when there is an update, `None` otherwise.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `kinematics`: Kinematics of the character.
    fn update(
        _controller_input: &ControllerInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        None
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
    };

    use super::{CharacterSequenceHandler, SequenceHandler};

    #[test]
    fn sequence_handler_default_update_is_empty() {
        // No update to run counter.
        let run_counter = None;
        // No update to HP.
        let hp = None;
        // No calculated next sequence.
        let sequence_id = None;
        // No update to sequence state.
        let sequence_state = None;
        // No update to facing direction.
        let mirrored = None;
        // No update to grounding.
        let grounding = None;
        assert_eq!(
            CharacterStatusUpdate::new(
                run_counter,
                hp,
                ObjectStatusUpdate::new(sequence_id, sequence_state, mirrored, grounding)
            ),
            Sit::update(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn sequence_handler_opt_default_update_is_none() {
        assert_eq!(
            None,
            Sleep::update(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &Kinematics::default()
            )
        );
    }

    struct Sit;
    impl CharacterSequenceHandler for Sit {}

    struct Sleep;
    impl SequenceHandler for Sleep {}
}

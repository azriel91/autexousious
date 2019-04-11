use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use game_input_model::ControlAction;
use specs_derive::Component;

use crate::{
    config::{self, SequenceId},
    loaded::{ControlTransitionHold, ControlTransitionPress, ControlTransitionRelease},
};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq, new)]
#[storage(VecStorage)]
pub enum ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// Transition to a specified sequence on control input press event.
    Press(ControlTransitionPress<SeqId>),
    /// Transition to a specified sequence on control input enabled state.
    Hold(ControlTransitionHold<SeqId>),
    /// Transition to a specified sequence on control input enabled state.
    Release(ControlTransitionRelease<SeqId>),
}

impl<SeqId> From<crate::config::ControlTransition<SeqId>> for ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    fn from(control_transition: config::ControlTransition<SeqId>) -> Self {
        use config::ControlTransition::*;
        match control_transition {
            PressDefend(sequence_id) => ControlTransition::Press(ControlTransitionPress {
                action: ControlAction::Defend,
                sequence_id,
            }),
            PressJump(sequence_id) => ControlTransition::Press(ControlTransitionPress {
                action: ControlAction::Jump,
                sequence_id,
            }),
            PressAttack(sequence_id) => ControlTransition::Press(ControlTransitionPress {
                action: ControlAction::Attack,
                sequence_id,
            }),
            PressSpecial(sequence_id) => ControlTransition::Press(ControlTransitionPress {
                action: ControlAction::Special,
                sequence_id,
            }),
            HoldDefend(sequence_id) => ControlTransition::Hold(ControlTransitionHold {
                action: ControlAction::Defend,
                sequence_id,
            }),
            HoldJump(sequence_id) => ControlTransition::Hold(ControlTransitionHold {
                action: ControlAction::Jump,
                sequence_id,
            }),
            HoldAttack(sequence_id) => ControlTransition::Hold(ControlTransitionHold {
                action: ControlAction::Attack,
                sequence_id,
            }),
            HoldSpecial(sequence_id) => ControlTransition::Hold(ControlTransitionHold {
                action: ControlAction::Special,
                sequence_id,
            }),
            ReleaseDefend(sequence_id) => ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Defend,
                sequence_id,
            }),
            ReleaseJump(sequence_id) => ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Jump,
                sequence_id,
            }),
            ReleaseAttack(sequence_id) => ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Attack,
                sequence_id,
            }),
            ReleaseSpecial(sequence_id) => ControlTransition::Release(ControlTransitionRelease {
                action: ControlAction::Special,
                sequence_id,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::ecs::{storage::DenseVecStorage, Component};
    use derivative::Derivative;
    use game_input_model::ControlAction;
    use serde::{Deserialize, Serialize};
    use specs_derive::Component;

    use super::ControlTransition;
    use crate::{
        config::{self, SequenceId},
        loaded::{ControlTransitionHold, ControlTransitionPress, ControlTransitionRelease},
    };

    macro_rules! from_test {
        ($test_name:ident, $mode_action:ident, $mode:ident, $action:ident, $mode_transition:ident) => {
            #[test]
            fn $test_name() {
                assert_eq!(
                    ControlTransition::$mode($mode_transition {
                        action: ControlAction::$action,
                        sequence_id: TestSeqId::Moo
                    }),
                    config::ControlTransition::$mode_action(TestSeqId::Moo).into()
                );
            }
        };
    }

    from_test!(
        from_press_defend,
        PressDefend,
        Press,
        Defend,
        ControlTransitionPress
    );
    from_test!(
        from_press_jump,
        PressJump,
        Press,
        Jump,
        ControlTransitionPress
    );
    from_test!(
        from_press_attack,
        PressAttack,
        Press,
        Attack,
        ControlTransitionPress
    );
    from_test!(
        from_press_special,
        PressSpecial,
        Press,
        Special,
        ControlTransitionPress
    );
    from_test!(
        from_release_defend,
        ReleaseDefend,
        Release,
        Defend,
        ControlTransitionRelease
    );
    from_test!(
        from_release_jump,
        ReleaseJump,
        Release,
        Jump,
        ControlTransitionRelease
    );
    from_test!(
        from_release_attack,
        ReleaseAttack,
        Release,
        Attack,
        ControlTransitionRelease
    );
    from_test!(
        from_release_special,
        ReleaseSpecial,
        Release,
        Special,
        ControlTransitionRelease
    );
    from_test!(
        from_hold_defend,
        HoldDefend,
        Hold,
        Defend,
        ControlTransitionHold
    );
    from_test!(from_hold_jump, HoldJump, Hold, Jump, ControlTransitionHold);
    from_test!(
        from_hold_attack,
        HoldAttack,
        Hold,
        Attack,
        ControlTransitionHold
    );
    from_test!(
        from_hold_special,
        HoldSpecial,
        Hold,
        Special,
        ControlTransitionHold
    );

    #[derive(
        Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize,
    )]
    #[derivative(Default)]
    enum TestSeqId {
        #[derivative(Default)]
        Boo,
        Moo,
    }
    impl SequenceId for TestSeqId {}
}

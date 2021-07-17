use character_model::config::CharacterSequenceName;
use derive_new::new;
use sequence_model::play::SequenceStatus;

use crate::CharacterSequenceUpdateComponents;

/// Switches to the ascend / descend sequence name when the current sequence
/// ends.
#[derive(Debug, new)]
pub struct SwitchSequenceOnEndYVelocity {
    /// The sequence to switch to if Y velocity is upwards.
    pub upwards: CharacterSequenceName,
    /// The sequence to switch to if Y velocity is downwards.
    pub downwards: CharacterSequenceName,
}

impl SwitchSequenceOnEndYVelocity {
    /// Switches to the ascend / descend sequence name when the current sequence
    /// ends.
    pub fn update(
        &self,
        components: CharacterSequenceUpdateComponents<'_>,
    ) -> Option<CharacterSequenceName> {
        if components.sequence_status == SequenceStatus::End {
            if components.velocity[1] > 0. {
                Some(self.upwards)
            } else {
                Some(self.downwards)
            }
        } else {
            None
        }
    }
}

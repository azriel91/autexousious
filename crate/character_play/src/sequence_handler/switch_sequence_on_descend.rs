use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

use crate::CharacterSequenceUpdateComponents;

/// `SwitchSequenceOnDescend` sequence update.
#[derive(Debug)]
pub struct SwitchSequenceOnDescend(
    /// The sequence to switch to.
    pub CharacterSequenceName,
);

impl SwitchSequenceOnDescend {
    /// Switches to the descend sequence name when Y velocity is downwards.
    pub fn update(
        &self,
        components: CharacterSequenceUpdateComponents<'_>,
    ) -> Option<CharacterSequenceName> {
        // Switch to descend_sequence when Y axis velocity is no longer upwards.
        if components.velocity[1] <= 0. {
            Some(self.0)
        } else if components.sequence_status == SequenceStatus::End {
            Some(components.character_sequence_name)
        } else {
            None
        }
    }
}

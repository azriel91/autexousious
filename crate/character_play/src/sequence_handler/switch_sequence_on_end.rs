use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

/// `SwitchSequenceOnEnd` sequence update.
#[derive(Debug)]
pub struct SwitchSequenceOnEnd(
    /// The sequence to switch to.
    pub CharacterSequenceName,
);

impl SwitchSequenceOnEnd {
    /// Switches to the specified sequence name when the current sequence ends.
    pub fn update(&self, sequence_status: SequenceStatus) -> Option<CharacterSequenceName> {
        if sequence_status == SequenceStatus::End {
            Some(self.0)
        } else {
            None
        }
    }
}

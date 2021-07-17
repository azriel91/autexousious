use character_model::config::CharacterSequenceName;
use object_model::play::Grounding;

use crate::CharacterSequenceUpdateComponents;

/// `SwitchSequenceOnLand` sequence update.
#[derive(Debug)]
pub struct SwitchSequenceOnLand(
    /// The sequence to switch to.
    pub CharacterSequenceName,
);

impl SwitchSequenceOnLand {
    /// Switches to the landing sequence name when the character is on ground.
    pub fn update(
        &self,
        components: CharacterSequenceUpdateComponents<'_>,
    ) -> Option<CharacterSequenceName> {
        if components.grounding == Grounding::OnGround {
            Some(self.0)
        } else {
            None
        }
    }
}

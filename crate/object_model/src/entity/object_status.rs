use std::ops::{Add, AddAssign};

use amethyst::ecs::{prelude::*, storage::DenseVecStorage};

use config::object::{SequenceId, SequenceState};
use entity::ObjectStatusUpdate;

/// Status of an object entity.
///
/// We use a `DenseVecStorage` because all object entities have their own type of `SequenceId`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct ObjectStatus<SeqId: SequenceId> {
    /// ID of the current sequence the entity is on.
    pub sequence_id: SeqId,
    /// Whether the sequence just started, is ongoing, or has ended.
    pub sequence_state: SequenceState,
    /// Whether or not this object is facing left.
    pub mirrored: bool,
}

impl<SeqId: SequenceId + 'static> Component for ObjectStatus<SeqId> {
    type Storage = DenseVecStorage<Self>;
}

impl<SeqId: SequenceId> Add<ObjectStatusUpdate<SeqId>> for ObjectStatus<SeqId> {
    type Output = Self;

    fn add(self, delta: ObjectStatusUpdate<SeqId>) -> Self {
        ObjectStatus {
            sequence_id: delta.sequence_id.unwrap_or(self.sequence_id),
            sequence_state: delta.sequence_state.unwrap_or(self.sequence_state),
            mirrored: delta.mirrored.unwrap_or(self.mirrored),
        }
    }
}

impl<SeqId: SequenceId> AddAssign<ObjectStatusUpdate<SeqId>> for ObjectStatus<SeqId> {
    fn add_assign(&mut self, delta: ObjectStatusUpdate<SeqId>) {
        *self = *self + delta;
    }
}

#[cfg(test)]
mod test {
    use config::object::{SequenceId, SequenceState};
    use entity::ObjectStatusUpdate;

    use super::ObjectStatus;

    #[test]
    fn add_retains_values_if_no_delta() {
        let status = ObjectStatus::new(TestSeqId::Boo, SequenceState::End, false);
        let delta = ObjectStatusUpdate::new(None, None, None);

        assert_eq!(
            ObjectStatus::new(TestSeqId::Boo, SequenceState::End, false),
            status + delta
        );
    }

    #[test]
    fn add_updates_sequence_id_if_present() {
        let status = ObjectStatus::new(TestSeqId::Boo, SequenceState::End, false);
        let delta = ObjectStatusUpdate::new(Some(TestSeqId::Moo), None, None);

        assert_eq!(
            ObjectStatus::new(TestSeqId::Moo, SequenceState::End, false),
            status + delta
        );
    }

    #[test]
    fn add_updates_sequence_state_if_present() {
        let status = ObjectStatus::new(TestSeqId::Boo, SequenceState::Ongoing, false);
        let delta = ObjectStatusUpdate::new(None, Some(SequenceState::End), None);

        assert_eq!(
            ObjectStatus::new(TestSeqId::Boo, SequenceState::End, false),
            status + delta
        );
    }

    #[test]
    fn add_updates_mirrored_if_present() {
        let status = ObjectStatus::new(TestSeqId::Boo, SequenceState::End, false);
        let delta = ObjectStatusUpdate::new(None, None, Some(true));

        assert_eq!(
            ObjectStatus::new(TestSeqId::Boo, SequenceState::End, true),
            status + delta
        );
    }

    #[test]
    fn add_retains_mirrored_when_delta_value_is_same() {
        let status = ObjectStatus::new(TestSeqId::Boo, SequenceState::End, true);
        let delta = ObjectStatusUpdate::new(None, Some(SequenceState::End), Some(true));

        assert_eq!(
            ObjectStatus::new(TestSeqId::Boo, SequenceState::End, true),
            status + delta
        );
    }

    #[test]
    fn add_assign_updates_fields_if_present() {
        let mut status = ObjectStatus::new(TestSeqId::Boo, SequenceState::Begin, false);
        let delta = ObjectStatusUpdate::new(
            Some(TestSeqId::Moo),
            Some(SequenceState::Ongoing),
            Some(true),
        );

        status += delta;
        assert_eq!(
            ObjectStatus::new(TestSeqId::Moo, SequenceState::Ongoing, true),
            status
        );
    }

    #[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash)]
    #[derivative(Default)]
    enum TestSeqId {
        #[derivative(Default)]
        Boo,
        Moo,
    }
    impl SequenceId for TestSeqId {}
}

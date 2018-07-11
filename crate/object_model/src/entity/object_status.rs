use std::ops::{Add, AddAssign};

use amethyst::ecs::{prelude::*, storage::DenseVecStorage};

use config::object::SequenceId;
use entity::ObjectStatusUpdate;

/// Status of an object entity.
///
/// We use a `DenseVecStorage` because all object entities have their own type of `SequenceId`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct ObjectStatus<SeqId: SequenceId> {
    /// ID of the current sequence the entity is on.
    pub sequence_id: SeqId,
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
    use config::object::SequenceId;
    use entity::ObjectStatusUpdate;

    use super::ObjectStatus;

    #[test]
    fn add_retains_values_if_no_delta() {
        let status = ObjectStatus::new(TestSeqId::Boo, false);
        let delta = ObjectStatusUpdate::new(None, None);

        assert_eq!(ObjectStatus::new(TestSeqId::Boo, false), status + delta);
    }

    #[test]
    fn add_updates_sequence_id_if_present() {
        let status = ObjectStatus::new(TestSeqId::Boo, false);
        let delta = ObjectStatusUpdate::new(Some(TestSeqId::Moo), None);

        assert_eq!(ObjectStatus::new(TestSeqId::Moo, false), status + delta);
    }

    #[test]
    fn add_updates_mirrored_if_present() {
        let status = ObjectStatus::new(TestSeqId::Boo, false);
        let delta = ObjectStatusUpdate::new(None, Some(true));

        assert_eq!(ObjectStatus::new(TestSeqId::Boo, true), status + delta);
    }

    #[test]
    fn add_retains_mirrored_when_delta_value_is_same() {
        let status = ObjectStatus::new(TestSeqId::Boo, true);
        let delta = ObjectStatusUpdate::new(None, Some(true));

        assert_eq!(ObjectStatus::new(TestSeqId::Boo, true), status + delta);
    }

    #[test]
    fn add_assign_updates_fields_if_present() {
        let mut status = ObjectStatus::new(TestSeqId::Boo, false);
        let delta = ObjectStatusUpdate::new(Some(TestSeqId::Moo), Some(true));

        status += delta;
        assert_eq!(ObjectStatus::new(TestSeqId::Moo, true), status);
    }

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
    enum TestSeqId {
        Boo,
        Moo,
    }
    impl Default for TestSeqId {
        fn default() -> Self {
            TestSeqId::Boo
        }
    }
    impl SequenceId for TestSeqId {}
}

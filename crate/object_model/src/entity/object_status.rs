use std::ops::{Add, AddAssign};

use amethyst::ecs::{
    storage::{DenseVecStorage, FlaggedStorage},
    Component,
};

use config::object::{SequenceId, SequenceState};
use entity::{Grounding, ObjectStatusUpdate};

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
    /// Tracks an object's attachment to the surrounding environment.
    pub grounding: Grounding,
}

impl<SeqId: SequenceId + 'static> Component for ObjectStatus<SeqId> {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl<SeqId: SequenceId> Add<ObjectStatusUpdate<SeqId>> for ObjectStatus<SeqId> {
    type Output = Self;

    fn add(self, delta: ObjectStatusUpdate<SeqId>) -> Self {
        ObjectStatus {
            sequence_id: delta.sequence_id.unwrap_or(self.sequence_id),
            sequence_state: delta.sequence_state.unwrap_or(self.sequence_state),
            mirrored: delta.mirrored.unwrap_or(self.mirrored),
            grounding: delta.grounding.unwrap_or(self.grounding),
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
    use entity::{Grounding, ObjectStatusUpdate};

    use super::ObjectStatus;

    #[test]
    fn add_retains_values_if_no_delta() {
        let status = ObjectStatus::new(
            TestSeqId::Moo,
            SequenceState::End,
            true,
            Grounding::Airborne,
        );
        let delta = ObjectStatusUpdate::default();

        assert_eq!(
            ObjectStatus::new(
                TestSeqId::Moo,
                SequenceState::End,
                true,
                Grounding::Airborne
            ),
            status + delta
        );
    }

    #[test]
    fn add_updates_sequence_id_if_present() {
        let status = ObjectStatus {
            sequence_id: TestSeqId::Boo,
            ..Default::default()
        };
        let delta = ObjectStatusUpdate {
            sequence_id: Some(TestSeqId::Moo),
            ..Default::default()
        };

        assert_eq!(TestSeqId::Moo, (status + delta).sequence_id);
    }

    #[test]
    fn add_updates_sequence_state_if_present() {
        let status = ObjectStatus::<TestSeqId> {
            sequence_state: SequenceState::Ongoing,
            ..Default::default()
        };
        let delta = ObjectStatusUpdate {
            sequence_state: Some(SequenceState::End),
            ..Default::default()
        };

        assert_eq!(SequenceState::End, (status + delta).sequence_state);
    }

    #[test]
    fn add_updates_mirrored_if_present() {
        let status = ObjectStatus::<TestSeqId> {
            mirrored: false,
            ..Default::default()
        };
        let delta = ObjectStatusUpdate {
            mirrored: Some(true),
            ..Default::default()
        };

        assert_eq!(true, (status + delta).mirrored);
    }

    #[test]
    fn add_updates_grounding_if_present() {
        let status = ObjectStatus::<TestSeqId> {
            grounding: Grounding::Airborne,
            ..Default::default()
        };
        let delta = ObjectStatusUpdate {
            grounding: Some(Grounding::OnGround),
            ..Default::default()
        };

        assert_eq!(Grounding::OnGround, (status + delta).grounding);
    }

    #[test]
    fn add_retains_value_when_delta_value_is_same() {
        let status = ObjectStatus::new(
            TestSeqId::Boo,
            SequenceState::End,
            true,
            Grounding::Airborne,
        );
        let delta = ObjectStatusUpdate::new(
            Some(TestSeqId::Boo),
            Some(SequenceState::End),
            Some(true),
            Some(Grounding::Airborne),
        );

        assert_eq!(
            ObjectStatus::new(
                TestSeqId::Boo,
                SequenceState::End,
                true,
                Grounding::Airborne
            ),
            status + delta
        );
    }

    #[test]
    fn add_assign_updates_fields_if_present() {
        let mut status = ObjectStatus::new(
            TestSeqId::Boo,
            SequenceState::Begin,
            false,
            Grounding::Airborne,
        );
        let delta = ObjectStatusUpdate::new(
            Some(TestSeqId::Moo),
            Some(SequenceState::Ongoing),
            Some(true),
            Some(Grounding::OnGround),
        );

        status += delta;
        assert_eq!(
            ObjectStatus::new(
                TestSeqId::Moo,
                SequenceState::Ongoing,
                true,
                Grounding::OnGround
            ),
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

use std::ops::{Add, AddAssign};

use amethyst::ecs::{prelude::*, storage::DenseVecStorage};

use config::object::CharacterSequenceId;
use entity::{CharacterStatusUpdate, ObjectStatus, RunCounter};

/// Character-specific status for character entities.
///
/// We use a `DenseVecStorage` because all character entities, but not all entities will have this.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct CharacterStatus {
    /// Tracks state used to determine when a character should run.
    pub run_counter: RunCounter,
    /// Common object status for all object type entities.
    pub object_status: ObjectStatus<CharacterSequenceId>,
}

impl Component for CharacterStatus {
    type Storage = DenseVecStorage<Self>;
}

impl Add<CharacterStatusUpdate> for CharacterStatus {
    type Output = Self;

    fn add(self, update: CharacterStatusUpdate) -> Self {
        CharacterStatus {
            run_counter: update.run_counter.unwrap_or(self.run_counter),
            object_status: self.object_status + update.object_status,
        }
    }
}

impl AddAssign<CharacterStatusUpdate> for CharacterStatus {
    fn add_assign(&mut self, update: CharacterStatusUpdate) {
        *self = *self + update;
    }
}

#[cfg(test)]
mod test {
    use config::object::{CharacterSequenceId, SequenceState};
    use entity::{CharacterStatusUpdate, ObjectStatus, ObjectStatusUpdate, RunCounter};

    use super::CharacterStatus;

    #[test]
    fn add_retains_values_if_no_update() {
        let status = CharacterStatus::new(RunCounter::Increase(10), ObjectStatus::default());
        let update = CharacterStatusUpdate::new(None, ObjectStatusUpdate::default());

        assert_eq!(
            CharacterStatus::new(RunCounter::Increase(10), ObjectStatus::default()),
            status + update
        );
    }

    #[test]
    fn add_updates_run_counter_if_present() {
        let status = CharacterStatus::new(RunCounter::Increase(10), ObjectStatus::default());
        let update = CharacterStatusUpdate::new(
            Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
            ObjectStatusUpdate::default(),
        );

        assert_eq!(
            CharacterStatus::new(
                RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                ObjectStatus::default()
            ),
            status + update
        );
    }

    #[test]
    fn add_updates_object_status() {
        let status = CharacterStatus::new(RunCounter::Increase(10), ObjectStatus::default());
        let update = CharacterStatusUpdate::new(
            Some(RunCounter::Increase(9)),
            ObjectStatusUpdate::new(
                Some(CharacterSequenceId::Walk),
                Some(SequenceState::End),
                Some(true),
            ),
        );

        assert_eq!(
            CharacterStatus::new(
                RunCounter::Increase(9),
                ObjectStatus::new(CharacterSequenceId::Walk, SequenceState::End, true)
            ),
            status + update
        );
    }

    #[test]
    fn add_assign_updates_fields_if_present() {
        let mut status = CharacterStatus::new(RunCounter::Increase(10), ObjectStatus::default());
        let update = CharacterStatusUpdate::new(
            Some(RunCounter::Increase(9)),
            ObjectStatusUpdate::new(
                Some(CharacterSequenceId::Walk),
                Some(SequenceState::End),
                Some(true),
            ),
        );

        status += update;
        assert_eq!(
            CharacterStatus::new(
                RunCounter::Increase(9),
                ObjectStatus::new(CharacterSequenceId::Walk, SequenceState::End, true)
            ),
            status
        );
    }
}

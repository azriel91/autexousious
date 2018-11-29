use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, ObjectStatusUpdate, SequenceStatus},
};

use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnLand(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnLand {
    pub fn update<'c>(
        &self,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();
        if components.grounding == Grounding::OnGround {
            object_status_update.sequence_id = Some(self.0);
        } else if components.sequence_status == SequenceStatus::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::FallForwardDescend);
        }

        object_status_update
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::SwitchSequenceOnLand;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate::default(),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                    },
                    SequenceStatus::default(),
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::FallForwardDescend),
            },
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                    },
                    SequenceStatus::End,
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn switches_to_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::FallForwardLand),
            },
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                    },
                    SequenceStatus::default(),
                    &kinematics,
                    Mirrored::default(),
                    Grounding::OnGround,
                    RunCounter::default()
                )
            )
        );
    }
}

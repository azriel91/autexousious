mod common;
mod dash_attack;
mod dodge;
mod jump;
mod jump_attack;
mod jump_descend_land;
mod run;
mod run_stop;
mod sequence_handler_util;
mod stand;
mod switch_sequence_on_descend;
mod switch_sequence_on_end;
mod switch_sequence_on_end_y_velocity;
mod switch_sequence_on_land;
mod walk;

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
    };

    #[test]
    fn sequence_handler_default_update_is_none() {
        assert_eq!(
            None,
            Sit::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::default(),
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    struct Sit;
    impl CharacterSequenceHandler for Sit {}
}

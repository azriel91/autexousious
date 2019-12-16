#[cfg(test)]
mod tests {
    use game_play_model::{GamePlayEvent, GamePlayEventArgs};
    use stdio_spi::StdinMapper;

    use game_play_stdio::GamePlayEventStdinMapper;

    macro_rules! test_mapping {
        ($test_name:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                let args = GamePlayEventArgs::$variant;

                let result = GamePlayEventStdinMapper::map(&(), args);

                assert!(result.is_ok());
                assert_eq!(GamePlayEvent::$variant, result.unwrap())
            }
        };
    }

    test_mapping!(maps_return_event, Return);
    test_mapping!(maps_restart_event, Restart);
    test_mapping!(maps_pause_event, Pause);
    test_mapping!(maps_resume_event, Resume);
    test_mapping!(maps_end_event, End);
    test_mapping!(maps_end_stats_event, EndStats);
}

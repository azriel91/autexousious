#[cfg(test)]
mod test {
    use amethyst::{GameData, State, Trans};
    use application_event::AppEvent;
    use debug_util_amethyst::assert_eq_trans;
    use game_mode_selection_model::GameModeIndex;

    use game_mode_selection::GameModeSelectionTrans;

    #[test]
    fn trans_returns_push_for_start_game() {
        assert_eq_trans(
            &Trans::Push(Box::new(MockState)),
            &GameModeSelectionTrans::trans(GameModeIndex::StartGame),
        );
    }

    #[test]
    fn trans_returns_quit_for_exit() {
        assert_eq_trans(
            &Trans::Quit as &Trans<_, _>,
            &GameModeSelectionTrans::trans(GameModeIndex::Exit),
        );
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for MockState {}
}

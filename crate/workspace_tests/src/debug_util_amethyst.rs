#[cfg(test)]
mod test {
    use amethyst::{GameData, State, Trans};

    use debug_util_amethyst::{assert_eq_opt_trans, assert_eq_trans};

    #[macro_use]
    macro_rules! test_opt_trans_panic {
        ($test_name:ident, $message:expr, $expected:expr, $actual:expr) => {
            #[test]
            #[should_panic(expected = $message)]
            fn $test_name() {
                assert_eq_opt_trans::<GameData<'static, 'static>, ()>($expected, $actual);
            } // kcov-ignore
        };
    }

    #[test]
    fn assert_eq_trans_does_not_panic_on_same_trans_discriminant() {
        assert_eq_trans::<GameData<'static, 'static>, ()>(&Trans::None, &Trans::None);
        assert_eq_trans(
            &Trans::Push(Box::new(MockState)),
            &Trans::Push(Box::new(MockState)),
        ); // kcov-ignore
    }

    #[test]
    #[should_panic(expected = "Expected `None` but got `Push`.")]
    fn assert_eq_trans_panics_on_different_trans_discriminant() {
        assert_eq_trans(&Trans::None, &Trans::Push(Box::new(MockState)));
    } // kcov-ignore

    #[test]
    fn assert_eq_opt_trans_does_not_panic_on_none_none() {
        assert_eq_opt_trans::<GameData<'static, 'static>, ()>(None, None);
    }

    #[test]
    fn assert_eq_opt_trans_does_not_panic_on_same_discriminant() {
        assert_eq_opt_trans::<GameData<'static, 'static>, ()>(
            Some(Trans::None).as_ref(),
            Some(Trans::None).as_ref(),
        );
        assert_eq_opt_trans(
            Some(Trans::Push(Box::new(MockState))).as_ref(),
            Some(Trans::Push(Box::new(MockState))).as_ref(),
        ); // kcov-ignore
    }

    test_opt_trans_panic!(
        assert_eq_opt_trans_panics_on_some_none,
        "Expected `Some(Pop)` but got `None`.",
        Some(Trans::Pop).as_ref(),
        None
    );

    test_opt_trans_panic!(
        assert_eq_opt_trans_panics_on_none_some,
        "Expected `None` but got `Some(Pop)`.",
        None,
        Some(Trans::Pop).as_ref()
    );

    test_opt_trans_panic!(
        assert_eq_opt_trans_panics_on_different_trans_discriminant,
        "Expected `Some(Pop)` but got `Some(Push)`.",
        Some(Trans::Pop).as_ref(),
        Some(Trans::Push(Box::new(MockState))).as_ref()
    );

    struct MockState;
    impl State<GameData<'static, 'static>, ()> for MockState {}
}

#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides functions to format [Amethyst][amethyst] types as strings.
//!
//! [amethyst]: https://github.com/amethyst/amethyst

extern crate amethyst;

use std::mem::discriminant;

use amethyst::prelude::*;

/// Returns a string representation of the `Trans`' discriminant.
///
/// # Parameters
///
/// * `trans`: `Trans` to display.
///
/// # Examples
///
/// ```rust
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::display_trans;
/// #
/// # struct MyState;
/// # impl State<GameData<'static, 'static>, ()> for MyState {}
/// #
/// # fn main() {
/// let trans = Trans::Push(Box::new(MyState));
/// assert_eq!("Trans::Push", display_trans(&trans));
/// # }
/// ```
pub fn display_trans<T, E>(trans: &Trans<T, E>) -> &str {
    match *trans {
        Trans::None => "Trans::None",
        Trans::Quit => "Trans::Quit",
        Trans::Pop => "Trans::Pop",
        Trans::Push(..) => "Trans::Push",
        Trans::Switch(..) => "Trans::Switch",
    }
} // kcov-ignore

/// Asserts that the `Trans` objects, disregarding their `State`, are equal.
///
/// This `panic!`s with a readable error message when the assertion fails.
///
/// # Parameters
///
/// * `expected`: The `Trans` that is desired.
/// * `actual`: The `Trans` that was acquired.
///
/// # Examples
///
/// Successful assertion:
///
/// ```rust
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::assert_eq_trans;
/// #
/// # fn main() {
/// // ok
/// assert_eq_trans::<(), ()>(&Trans::None, &Trans::None);
/// # }
/// ```
///
/// Failing assertion:
///
/// ```rust,should_panic
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::assert_eq_trans;
/// #
/// # fn main() {
/// // panic: Expected `Trans::None` but got `Trans::Pop`.
/// assert_eq_trans::<(), ()>(&Trans::None, &Trans::Pop);
/// # }
/// ```
///
/// # Panics
///
/// When the expected and actual `Trans` differ.
pub fn assert_eq_trans<T, E>(expected: &Trans<T, E>, actual: &Trans<T, E>) {
    assert_eq!(
        discriminant(expected),
        discriminant(actual),
        "Expected `{}` but got `{}`.",
        display_trans(expected),
        display_trans(actual)
    );
}

/// Asserts that the `Trans` objects contained in the `Option`s, disregarding their `State`, are
/// equal.
///
/// This `panic!`s with a readable error message when the assertion fails.
///
/// # Parameters
///
/// * `expected`: The `Option<Trans>` that is desired.
/// * `actual`: The `Option<Trans>` that was acquired.
///
/// # Examples
///
/// Successful assertion:
///
/// ```rust
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::assert_eq_opt_trans;
/// #
/// # fn main() {
/// assert_eq_opt_trans::<(), ()>(None, None);
/// assert_eq_opt_trans::<(), ()>(Some(Trans::None).as_ref(), Some(Trans::None).as_ref());
/// # }
/// ```
///
/// Failing assertion:
///
/// ```rust,should_panic
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::assert_eq_opt_trans;
/// #
/// # fn main() {
/// // panic: Expected `Some(Trans::None)` but got `Some(Trans::Pop)`.
/// assert_eq_opt_trans::<(), ()>(Some(Trans::None).as_ref(), Some(Trans::Pop).as_ref());
/// # }
/// ```
///
/// # Panics
///
/// When the expected and actual `Trans` differ.
pub fn assert_eq_opt_trans<T, E>(expected: Option<&Trans<T, E>>, actual: Option<&Trans<T, E>>) {
    match expected {
        Some(expected) => match actual {
            Some(actual) => {
                assert_eq!(
                    discriminant(expected),
                    discriminant(actual),
                    "Expected `Some({})` but got `Some({})`.",
                    display_trans(expected),
                    display_trans(actual)
                );
            }
            None => panic!(
                "Expected `Some({})` but got `None`.",
                display_trans(expected)
            ),
        },
        None => {
            if let Some(actual) = actual {
                panic!("Expected `None` but got `Some({})`.", display_trans(actual));
            }
        }
    };
}

#[cfg(test)]
mod test {
    use amethyst::prelude::*;

    use super::{assert_eq_opt_trans, assert_eq_trans, display_trans};

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
    fn display_trans_none() {
        assert_eq!(
            "Trans::None",
            display_trans::<GameData<'static, 'static>, ()>(&Trans::None)
        );
    }

    #[test]
    fn display_trans_quit() {
        assert_eq!(
            "Trans::Quit",
            display_trans::<GameData<'static, 'static>, ()>(&Trans::Quit)
        );
    }

    #[test]
    fn display_trans_pop() {
        assert_eq!(
            "Trans::Pop",
            display_trans::<GameData<'static, 'static>, ()>(&Trans::Pop)
        );
    }

    #[test]
    fn display_trans_push() {
        assert_eq!(
            "Trans::Push",
            display_trans(&Trans::Push(Box::new(MockState)))
        );
    }

    #[test]
    fn display_trans_switch() {
        assert_eq!(
            "Trans::Switch",
            display_trans(&Trans::Switch(Box::new(MockState)))
        );
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
    #[should_panic(expected = "Expected `Trans::None` but got `Trans::Push`.")]
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
        "Expected `Some(Trans::Pop)` but got `None`.",
        Some(Trans::Pop).as_ref(),
        None
    );

    test_opt_trans_panic!(
        assert_eq_opt_trans_panics_on_none_some,
        "Expected `None` but got `Some(Trans::Pop)`.",
        None,
        Some(Trans::Pop).as_ref()
    );

    test_opt_trans_panic!(
        assert_eq_opt_trans_panics_on_different_trans_discriminant,
        "Expected `Some(Trans::Pop)` but got `Some(Trans::Push)`.",
        Some(Trans::Pop).as_ref(),
        Some(Trans::Push(Box::new(MockState))).as_ref()
    );

    struct MockState;
    impl State<GameData<'static, 'static>, ()> for MockState {}
}

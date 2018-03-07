#![deny(missing_docs)]
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
/// # impl State for MyState {}
/// #
/// # fn main() {
/// let trans = Trans::Push(Box::new(MyState));
/// assert_eq!("Trans::Push", display_trans(&trans));
/// # }
/// ```
pub fn display_trans(trans: &Trans) -> &str {
    match *trans {
        Trans::None => "Trans::None",
        Trans::Quit => "Trans::Quit",
        Trans::Pop => "Trans::Pop",
        Trans::Push(..) => "Trans::Push",
        Trans::Switch(..) => "Trans::Switch",
    }
}

/// Asserts that the `Trans` objects, but not their state, are equal.
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
/// ```rust,should_panic
/// # extern crate amethyst;
/// # extern crate debug_util_amethyst;
/// #
/// # use amethyst::prelude::*;
/// # use debug_util_amethyst::assert_eq_trans;
/// #
/// # fn main() {
/// // ok
/// assert_eq_trans(&Trans::None, &Trans::None);
///
/// // panic: Expected `Trans::None` but got `Trans::Pop`.
/// assert_eq_trans(&Trans::None, &Trans::Pop);
/// # }
/// ```
///
/// # Panics
///
/// When the expected and actual `Trans` differ.
pub fn assert_eq_trans(expected: &Trans, actual: &Trans) {
    assert_eq!(
        discriminant(expected),
        discriminant(actual),
        "Expected `{}` but got `{}`.",
        display_trans(expected),
        display_trans(actual)
    );
}

#[cfg(test)]
mod test {
    use amethyst::prelude::*;

    use super::{assert_eq_trans, display_trans};

    #[test]
    fn display_trans_none() {
        assert_eq!("Trans::None", display_trans(&Trans::None));
    }

    #[test]
    fn display_trans_quit() {
        assert_eq!("Trans::Quit", display_trans(&Trans::Quit));
    }

    #[test]
    fn display_trans_pop() {
        assert_eq!("Trans::Pop", display_trans(&Trans::Pop));
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
        assert_eq_trans(&Trans::None, &Trans::None);
        assert_eq_trans(
            &Trans::Push(Box::new(MockState)),
            &Trans::Push(Box::new(MockState)),
        );
    }

    #[test]
    #[should_panic(expected = "Expected `Trans::None` but got `Trans::Push`.")]
    fn assert_eq_trans_panics_on_different_trans_discriminant() {
        assert_eq_trans(&Trans::None, &Trans::Push(Box::new(MockState)));
    }

    struct MockState;
    impl State for MockState {}
}

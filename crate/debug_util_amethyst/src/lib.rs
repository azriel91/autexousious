#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides functions to format [Amethyst][amethyst] types as strings.
//!
//! [amethyst]: https://github.com/amethyst/amethyst

use std::mem::discriminant;

use amethyst::Trans;

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

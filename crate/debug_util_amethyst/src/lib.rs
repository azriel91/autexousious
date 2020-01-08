#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides functions to format [Amethyst][amethyst] types as strings.
//!
//! [amethyst]: https://github.com/amethyst/amethyst

use std::mem::discriminant;

use amethyst::Trans;

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
/// // ok
/// assert_eq_trans::<(), ()>(&Trans::None, &Trans::None);
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
/// // panic: Expected `Trans::None` but got `Trans::Pop`.
/// assert_eq_trans::<(), ()>(&Trans::None, &Trans::Pop);
/// ```
///
/// # Panics
///
/// When the expected and actual `Trans` differ.
pub fn assert_eq_trans<T, E>(expected: &Trans<T, E>, actual: &Trans<T, E>) {
    assert_eq!(
        discriminant(expected),
        discriminant(actual),
        "Expected `{:?}` but got `{:?}`.",
        expected,
        actual
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
/// assert_eq_opt_trans::<(), ()>(None, None);
/// assert_eq_opt_trans::<(), ()>(Some(Trans::None).as_ref(), Some(Trans::None).as_ref());
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
/// // panic: Expected `Some(Trans::None)` but got `Some(Trans::Pop)`.
/// assert_eq_opt_trans::<(), ()>(Some(Trans::None).as_ref(), Some(Trans::Pop).as_ref());
/// ```
///
/// # Panics
///
/// When the expected and actual `Trans` differ.
pub fn assert_eq_opt_trans<T, E>(expected: Option<&Trans<T, E>>, actual: Option<&Trans<T, E>>) {
    match (expected, actual) {
        (Some(expected_trans), Some(actual_trans)) => assert_eq!(
            discriminant(expected_trans),
            discriminant(actual_trans),
            "Expected `{:?}` but got `{:?}`.",
            expected,
            actual
        ),
        (Some(_), None) => panic!("Expected `{:?}` but got `None`.", expected),
        (None, Some(_)) => panic!("Expected `None` but got `{:?}`.", actual),
        (None, None) => {}
    };
}

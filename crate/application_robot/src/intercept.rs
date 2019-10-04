pub use self::{
    application_event_intercept::ApplicationEventIntercept,
    fixed_timeout_intercept::FixedTimeoutIntercept,
    keyboard_escape_intercept::KeyboardEscapeIntercept,
};

mod application_event_intercept;
mod fixed_timeout_intercept;
mod keyboard_escape_intercept;

use std::fmt::Debug;

use amethyst::{StateData, Trans};

// Ignore default implementation coverage.
// kcov-ignore-start
/// Trait for types that intercept and manipulate application behaviour.
///
/// Types that implement this trait are invoked at the beginning of each [`State`][state] function
/// so they may record application state or override the behaviour of the state.
///
/// [state]: https://docs.rs/amethyst/0.6.0/amethyst/trait.State.html
pub trait Intercept<T, E>: Debug
where
    E: Send + Sync + 'static,
{
    /// Invoked before the delegate state's `on_start(..)` invocation.
    fn on_start_begin(&mut self, _data: &mut StateData<'_, T>) {}
    /// Invoked after the delegate state's `on_start(..)` invocation.
    fn on_start_end(&mut self) {}
    /// Invoked before the delegate state's `on_stop(..)` invocation.
    fn on_stop_begin(&mut self, _data: &mut StateData<'_, T>) {}
    /// Invoked after the delegate state's `on_stop(..)` invocation.
    fn on_stop_end(&mut self) {}
    /// Invoked before the delegate state's `on_pause(..)` invocation.
    fn on_pause_begin(&mut self, _data: &mut StateData<'_, T>) {}
    /// Invoked after the delegate state's `on_pause(..)` invocation.
    fn on_pause_end(&mut self) {}
    /// Invoked before the delegate state's `on_resume(..)` invocation.
    fn on_resume_begin(&mut self, _data: &mut StateData<'_, T>) {}
    /// Invoked after the delegate state's `on_resume(..)` invocation.
    fn on_resume_end(&mut self) {}
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `handle_event(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `data`: `StateData` for the application `State`
    /// * `event`: `Event` received by the application. Mutable so the `Intercept` may alter
    ///             behaviour.
    fn handle_event_begin(
        &mut self,
        _data: &mut StateData<'_, T>,
        _event: &mut E,
    ) -> Option<Trans<T, E>> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `handle_event(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn handle_event_end(&mut self, _state_trans: &Trans<T, E>) -> Option<Trans<T, E>> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `fixed_update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `data`: `StateData` for the application `State`.
    fn fixed_update_begin(&mut self, _data: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `fixed_update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn fixed_update_end(&mut self, _state_trans: &Trans<T, E>) -> Option<Trans<T, E>> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `data`: `StateData` for the application `State`.
    fn update_begin(&mut self, _data: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn update_end(&mut self, _state_trans: &Trans<T, E>) -> Option<Trans<T, E>> {
        None
    }
    /// Returns whether this intercept should be shared with `Trans::Push` and `Trans::Switch`ed
    /// `State`s.
    ///
    /// Defaults to `false`.
    fn is_transitive(&self) -> bool {
        false
    }
}
// kcov-ignore-end

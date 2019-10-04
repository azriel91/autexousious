use std::time::{Duration, Instant};

use amethyst::{StateData, Trans};

use crate::Intercept;

/// Pops the application stack after a specified timeout.
///
/// This pops the stack after the fixed timeout, regardless of the state changes that occur from
/// the underlying delegate.
// kcov-ignore-start
#[derive(Debug)]
pub struct FixedTimeoutIntercept {
    /// Total duration that the delegate state should run for.
    pub timeout: Duration,
    /// Instant that the clock started ticking.
    pub start_instant: Option<Instant>,
}
// kcov-ignore-end

impl FixedTimeoutIntercept {
    /// Returns a new FixedTimeoutIntercept.
    ///
    /// # Parameters
    ///
    /// * `timeout`: Duration that the delegate state is permitted to run for.
    pub fn new(timeout: Duration) -> Self {
        FixedTimeoutIntercept {
            timeout,
            start_instant: None,
        }
    }

    fn pop_on_timeout<T, E>(&mut self) -> Option<Trans<T, E>> {
        // If start_instant is none, then it must have been popped by one of the pushed `State`s.
        if self.start_instant.is_none()
            || self.start_instant.as_ref().unwrap().elapsed() >= self.timeout
        {
            Some(Trans::Pop)
        } else {
            None
        }
    }
}

impl<T, E> Intercept<T, E> for FixedTimeoutIntercept
where
    E: Send + Sync + 'static,
{
    fn on_start_end(&mut self) {
        self.start_instant = Some(Instant::now());
    }

    fn fixed_update_begin(&mut self, _: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        self.pop_on_timeout()
    }

    fn update_begin(&mut self, _: &mut StateData<'_, T>) -> Option<Trans<T, E>> {
        self.pop_on_timeout()
    }

    fn on_stop_begin(&mut self, _: &mut StateData<'_, T>) {
        self.start_instant = None;
    }

    fn is_transitive(&self) -> bool {
        true
    }
}

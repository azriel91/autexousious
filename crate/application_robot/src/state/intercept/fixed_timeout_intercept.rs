use std::time::{Duration, Instant};

use amethyst::prelude::*;

use state::Intercept;

/// Pops the application stack after a specified timeout.
///
/// This pops the stack after the fixed timeout, regardless of the state changes that occur from
/// the underlying delegate.
///
/// TODO: implement wrapping state from `Trans::Push` with another `FixedTimeoutIntercept`.
#[derive(Debug)]
pub struct FixedTimeoutIntercept {
    /// Total duration that the delegate state should run for.
    timeout: Duration,
    /// Instant that the clock started ticking.
    start_instant: Option<Instant>,
}

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

    fn pop_on_timeout(&mut self) -> Option<Trans> {
        if self.start_instant
            .as_ref()
            .expect("Expected `self.start_instant` to be set.")
            .elapsed() >= self.timeout
        {
            Some(Trans::Pop)
        } else {
            None
        }
    }
}

impl Intercept for FixedTimeoutIntercept {
    fn on_start_end(&mut self, _: &mut World) {
        self.start_instant = Some(Instant::now());
    }

    fn fixed_update_begin(&mut self, _: &mut World) -> Option<Trans> {
        self.pop_on_timeout()
    }

    fn update_begin(&mut self, _: &mut World) -> Option<Trans> {
        self.pop_on_timeout()
    }

    fn on_stop_begin(&mut self, _: &mut World) {
        self.start_instant = None;
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use amethyst::prelude::*;
    use debug_util_amethyst::assert_eq_opt_trans;

    use state::Intercept;
    use super::FixedTimeoutIntercept;

    fn setup(timeout: Duration) -> (FixedTimeoutIntercept, World) {
        let world = World::new();
        (FixedTimeoutIntercept::new(timeout), world)
    }

    #[test]
    fn on_start_end_starts_timer() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        assert!(intercept.start_instant.is_none());

        intercept.on_start_end(&mut world);

        assert!(intercept.start_instant.is_some());
    }

    #[test]
    fn on_stop_begin_clears_timer() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        intercept.on_start_end(&mut world);

        assert!(intercept.start_instant.is_some());

        intercept.on_stop_begin(&mut world);

        assert!(intercept.start_instant.is_none());
    }

    #[test]
    fn fixed_update_begin_returns_none_on_non_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(10000));

        // Initialize start time
        intercept.on_start_end(&mut world);

        assert_eq_opt_trans(None, intercept.fixed_update_begin(&mut world).as_ref());
    }

    #[test]
    fn update_begin_returns_none_on_non_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(10000));

        // Initialize start time
        intercept.on_start_end(&mut world);

        assert_eq_opt_trans(None, intercept.update_begin(&mut world).as_ref());
    }

    #[test]
    fn fixed_update_begin_returns_trans_pop_on_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        // Initialize start time
        intercept.on_start_end(&mut world);

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref(),
            intercept.fixed_update_begin(&mut world).as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn update_begin_returns_trans_pop_on_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        // Initialize start time
        intercept.on_start_end(&mut world);

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref(),
            intercept.update_begin(&mut world).as_ref(),
        ); // kcov-ignore
    }
}

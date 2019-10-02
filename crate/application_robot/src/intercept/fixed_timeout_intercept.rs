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
    timeout: Duration,
    /// Instant that the clock started ticking.
    start_instant: Option<Instant>,
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

#[cfg(test)]
mod test {
    use std::time::Duration;

    use amethyst::prelude::*;
    use debug_util_amethyst::assert_eq_opt_trans;

    use super::FixedTimeoutIntercept;
    use crate::Intercept;

    fn setup(timeout: Duration) -> (FixedTimeoutIntercept, World) {
        let world = World::new();
        (FixedTimeoutIntercept::new(timeout), world)
    }

    #[test]
    fn on_start_end_starts_timer() {
        let (mut intercept, _world) = setup(Duration::from_millis(0));

        assert!(intercept.start_instant.is_none());

        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert!(intercept.start_instant.is_some());
    }

    #[test]
    fn on_stop_begin_clears_timer() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert!(intercept.start_instant.is_some());

        <dyn Intercept<(), ()>>::on_stop_begin(
            &mut intercept,
            &mut StateData::new(&mut world, &mut ()),
        );

        assert!(intercept.start_instant.is_none());
    }

    #[test]
    fn fixed_update_begin_returns_none_on_non_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(10000));

        // Initialize start time
        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert_eq_opt_trans(
            None as Option<&Trans<(), ()>>,
            intercept
                .fixed_update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn update_begin_returns_none_on_non_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(10000));

        // Initialize start time
        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert_eq_opt_trans(
            None as Option<&Trans<(), ()>>,
            intercept
                .update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn fixed_update_begin_returns_trans_pop_on_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        // Initialize start time
        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref() as Option<&Trans<(), ()>>,
            intercept
                .fixed_update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn update_begin_returns_trans_pop_on_timeout() {
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        // Initialize start time
        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref() as Option<&Trans<(), ()>>,
            intercept
                .update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn pop_on_timeout_returns_trans_pop_if_start_time_is_empty() {
        // This case happens when this intercept has been used by a State pushed by the wrapped
        // state.
        let (mut intercept, mut world) = setup(Duration::from_millis(0));

        // Initialize start time
        Intercept::<(), ()>::on_start_end(&mut intercept);

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref() as Option<&Trans<(), ()>>,
            intercept
                .update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore

        assert_eq_opt_trans(
            Some(Trans::Pop).as_ref() as Option<&Trans<(), ()>>,
            intercept
                .update_begin(&mut StateData::new(&mut world, &mut ()))
                .as_ref(),
        ); // kcov-ignore
    }

    #[test]
    fn intercept_is_transitive() {
        assert!(<dyn Intercept<(), ()>>::is_transitive(
            &FixedTimeoutIntercept::new(Duration::from_millis(0))
        ));
    }
}

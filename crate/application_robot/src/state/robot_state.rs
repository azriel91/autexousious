use std::{cell::RefCell, rc::Rc};

use amethyst::{State, StateData, Trans};
use derivative::Derivative;

use crate::{ApplicationEventIntercept, Intercept};

/// Wraps a delegate state with automation capabilities.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct RobotState<T, E>
where
    T: 'static,
    E: Send + Sync + 'static,
{
    /// Intercepts to track and control application behaviour.
    ///
    /// Rc<RefCell<Intercept<T, E>>> is a trait object, which does not implement Sized, needed by the generated
    /// setter from the `Builder` derive, so we instead provide default intercepts, and functions
    /// to toggle the enablement of certain `Intercept`s.
    #[derivative(Debug = "ignore")]
    pub intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>>,
    /// State to delegate behaviour to.
    #[derivative(Debug = "ignore")]
    pub delegate: Box<dyn State<T, E>>,
}

impl<T, E> RobotState<T, E>
where
    T: 'static,
    E: Send + Sync + 'static,
{
    /// Returns a new application robot state.
    pub fn new(delegate: Box<dyn State<T, E>>) -> Self {
        RobotState {
            intercepts: RobotState::default_intercepts(),
            delegate,
        } // kcov-ignore
    }

    /// Returns a new application robot state with only the specified intercepts.
    pub fn new_with_intercepts(
        delegate: Box<dyn State<T, E>>,
        intercepts: Vec<Rc<RefCell<dyn Intercept<T, E>>>>,
    ) -> Self {
        RobotState {
            intercepts,
            delegate,
        } // kcov-ignore
    }

    /// Returns the default intercepts for a `RobotState`.
    ///
    /// Currently this only includes the `ApplicationEventIntercept`.
    pub fn default_intercepts() -> Vec<Rc<RefCell<dyn Intercept<T, E>>>> {
        vec![Rc::new(RefCell::new(ApplicationEventIntercept::new()))]
    }

    fn fold_trans_begin<F>(&mut self, intercept_fn: F) -> Option<Trans<T, E>>
    where
        F: FnMut(&mut Rc<RefCell<dyn Intercept<T, E>>>) -> Option<Trans<T, E>>,
    {
        let trans_opt = self.intercepts.iter_mut().find_map(intercept_fn);

        trans_opt.map(|trans| self.wrap_trans(trans))
    }

    fn fold_trans_end<F>(&mut self, state_trans: Trans<T, E>, mut intercept_fn: F) -> Trans<T, E>
    where
        F: FnMut(&mut Rc<RefCell<dyn Intercept<T, E>>>, &Trans<T, E>) -> Option<Trans<T, E>>,
    {
        let intercept_trans = {
            let state_trans_ref = &state_trans;
            self.intercepts
                .iter_mut()
                .find_map(|intercept| intercept_fn(intercept, state_trans_ref))
        };
        self.wrap_trans(intercept_trans.unwrap_or(state_trans))
    }

    /// When returning a `Trans` with a `State`, wrap it with a `RobotState` with the transitive
    /// intercepts.
    fn wrap_trans(&mut self, trans: Trans<T, E>) -> Trans<T, E> {
        match trans {
            Trans::Push(state) => Trans::Push(self.wrap_trans_state(state)),
            Trans::Switch(state) => Trans::Switch(self.wrap_trans_state(state)),
            _ => trans,
        }
    }

    /// Returns the provided `trans_state` with a `RobotState` that shares this state's transitive
    /// `Intercept`s.
    ///
    /// # Parameters
    ///
    /// * `trans_state`: `State` that should be wrapped in a `RobotState`.
    fn wrap_trans_state(&mut self, trans_state: Box<dyn State<T, E>>) -> Box<dyn State<T, E>> {
        let intercepts = self
            .intercepts
            .iter()
            .filter(|intercept| intercept.borrow().is_transitive())
            .cloned()
            .collect::<Vec<Rc<RefCell<dyn Intercept<T, E>>>>>();
        Box::new(RobotState {
            intercepts,
            delegate: trans_state,
        })
    }
}

impl<'a, 'b, T, E> State<T, E> for RobotState<T, E>
where
    T: 'static,
    E: Send + Sync + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, T>) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_start_begin(&mut data));

        self.delegate.on_start(data);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_start_end());
    }

    fn on_stop(&mut self, mut data: StateData<'_, T>) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_stop_begin(&mut data));

        self.delegate.on_stop(data);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_stop_end());
    }

    fn on_pause(&mut self, mut data: StateData<'_, T>) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_pause_begin(&mut data));

        self.delegate.on_pause(data);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_pause_end());
    }

    fn on_resume(&mut self, mut data: StateData<'_, T>) {
        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_resume_begin(&mut data));

        self.delegate.on_resume(data);

        self.intercepts
            .iter_mut()
            .for_each(|intercept| intercept.borrow_mut().on_resume_end());
    }

    // TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/16>
    // kcov-ignore-start
    fn handle_event(&mut self, mut data: StateData<'_, T>, mut event: E) -> Trans<T, E> {
        let intercept_trans = self.fold_trans_begin(|intercept| {
            intercept
                .borrow_mut()
                .handle_event_begin(&mut data, &mut event)
        });
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.handle_event(data, event);

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.borrow_mut().handle_event_end(trans)
        })
    }
    // kcov-ignore-end

    fn fixed_update(&mut self, mut data: StateData<'_, T>) -> Trans<T, E> {
        let intercept_trans =
            self.fold_trans_begin(|intercept| intercept.borrow_mut().fixed_update_begin(&mut data));
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.fixed_update(data);

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.borrow_mut().fixed_update_end(trans)
        }) // kcov-ignore
    }

    fn update(&mut self, mut data: StateData<'_, T>) -> Trans<T, E> {
        let intercept_trans =
            self.fold_trans_begin(|intercept| intercept.borrow_mut().update_begin(&mut data));
        if let Some(trans) = intercept_trans {
            return trans;
        }

        let trans = self.delegate.update(data);

        self.fold_trans_end(trans, |intercept, trans| {
            intercept.borrow_mut().update_end(trans)
        }) // kcov-ignore
    }
}

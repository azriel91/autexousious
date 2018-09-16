use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::prelude::*;
use application_event::AppEvent;

use AppState;

/// Concrete type that implements `amethyst:State` to proxy through `AppState`s.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct StateProxy<'a, 'b, T>
where
    T: AppState<'a, 'b>,
{
    /// `AppState` to delegate to.
    #[derivative(Debug(bound = "T: Debug"))]
    pub delegate: T,
    /// `PhantomData` for lifetimes.
    marker: PhantomData<AppState<'a, 'b>>,
}

impl<'a, 'b, T> State<GameData<'a, 'b>, AppEvent> for StateProxy<'a, 'b, T>
where
    T: AppState<'a, 'b>,
{
    /// Executed when the game state begins.
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_start(data);
    }

    /// Executed when the game state exits.
    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_stop(data);
    }

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_pause(data);
    }

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.delegate.on_resume(data);
    }

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_event(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
        event: StateEvent<AppEvent>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        self.delegate.handle_event(data, event)
    }

    /// Executed repeatedly at stable, predictable intervals (1 / 60th of a second by default).
    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        self.delegate.fixed_update(data)
    }

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account
    /// the frame rate limit).
    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);
        self.delegate.update(data)
    }
}

impl<'a, 'b, T> From<T> for StateProxy<'a, 'b, T>
where
    T: AppState<'a, 'b>,
{
    fn from(app_state: T) -> Self {
        StateProxy::new(app_state)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use amethyst::{ecs::prelude::*, prelude::*};
    use application_event::AppEvent;
    use character_selection::CharacterSelectionEvent;

    use super::StateProxy;
    use AppState;

    macro_rules! test_delegate {
        ($test_name:ident, $method:ident, $expected:expr) => {
            #[test]
            fn $test_name() {
                let invocations = Rc::new(RefCell::new(vec![]));
                let mock_state = MockState::new(invocations.clone());
                let mut state_proxy = StateProxy::from(mock_state);
                let dispatcher = DispatcherBuilder::new().build();
                let mut game_data = GameData::new(dispatcher);
                let data = &mut game_data;
                let mut world = World::new();
                let world = &mut world;

                state_proxy.$method(StateData { world, data });

                assert_eq!($expected, invocations.borrow().iter().next());
            }
        };
    }

    test_delegate!(delegates_on_start, on_start, Some(&Invocation::OnStart));
    test_delegate!(delegates_on_stop, on_stop, Some(&Invocation::OnStop));
    test_delegate!(delegates_on_pause, on_pause, Some(&Invocation::OnPause));
    test_delegate!(delegates_on_resume, on_resume, Some(&Invocation::OnResume));
    test_delegate!(
        delegates_fixed_update,
        fixed_update,
        Some(&Invocation::FixedUpdate)
    );
    test_delegate!(delegates_update, update, Some(&Invocation::Update));

    #[test]
    fn delegates_handle_event() {
        let invocations = Rc::new(RefCell::new(vec![]));
        let mock_state = MockState::new(invocations.clone());
        let mut state_proxy = StateProxy::from(mock_state);
        let dispatcher = DispatcherBuilder::new().build();
        let mut game_data = GameData::new(dispatcher);
        let data = &mut game_data;
        let mut world = World::new();
        let world = &mut world;

        state_proxy.handle_event(
            StateData { world, data },
            StateEvent::Custom(AppEvent::CharacterSelection(
                CharacterSelectionEvent::Confirm,
            )),
        );

        assert_eq!(
            Some(&Invocation::HandleEvent),
            invocations.borrow().iter().next()
        );
    }

    #[derive(Debug, PartialEq)]
    enum Invocation {
        OnStart,
        OnStop,
        OnPause,
        OnResume,
        HandleEvent,
        FixedUpdate,
        Update,
    }

    type Invocations = Rc<RefCell<Vec<Invocation>>>;

    #[derive(Debug, Default, new)]
    struct MockState {
        invocations: Invocations,
    }

    impl<'a, 'b> AppState<'a, 'b> for MockState {
        fn on_start(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStart);
        }

        fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnStop);
        }

        fn on_pause(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnPause);
        }

        fn on_resume(&mut self, _data: StateData<GameData<'a, 'b>>) {
            self.invocations.borrow_mut().push(Invocation::OnResume);
        }

        fn handle_event(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
            _event: StateEvent<AppEvent>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::HandleEvent);
            Trans::None
        }

        fn fixed_update(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::FixedUpdate);
            Trans::None
        }

        fn update(
            &mut self,
            _data: StateData<GameData<'a, 'b>>,
        ) -> Trans<GameData<'a, 'b>, AppEvent> {
            self.invocations.borrow_mut().push(Invocation::Update);
            Trans::None
        }
    }
}

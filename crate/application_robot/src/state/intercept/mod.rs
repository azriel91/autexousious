pub use self::application_event_intercept::ApplicationEventIntercept;

mod application_event_intercept;

use std::fmt::Debug;

use amethyst::prelude::*;
use amethyst::renderer::Event;

// Ignore default implementation coverage.
// kcov-ignore-start
/// Trait for types that intercept and manipulate application behaviour.
///
/// Types that implement this trait are invoked at the beginning of each [`State`][state] function
/// so they may record application state or override the behaviour of the state.
///
/// [state]: https://docs.rs/amethyst/0.6.0/amethyst/trait.State.html
pub trait Intercept: Debug {
    /// Invoked before the delegate state's `on_start(..)` invocation.
    fn on_start_begin(&mut self, _world: &mut World) {}
    /// Invoked after the delegate state's `on_start(..)` invocation.
    fn on_start_end(&mut self, _world: &mut World) {}
    /// Invoked before the delegate state's `on_stop(..)` invocation.
    fn on_stop_begin(&mut self, _world: &mut World) {}
    /// Invoked after the delegate state's `on_stop(..)` invocation.
    fn on_stop_end(&mut self, _world: &mut World) {}
    /// Invoked before the delegate state's `on_pause(..)` invocation.
    fn on_pause_begin(&mut self, _world: &mut World) {}
    /// Invoked after the delegate state's `on_pause(..)` invocation.
    fn on_pause_end(&mut self, _world: &mut World) {}
    /// Invoked before the delegate state's `on_resume(..)` invocation.
    fn on_resume_begin(&mut self, _world: &mut World) {}
    /// Invoked after the delegate state's `on_resume(..)` invocation.
    fn on_resume_end(&mut self, _world: &mut World) {}
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `handle_event(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    /// * `event`: `Event` received by the application. Mutable so the `Intercept` may alter
    ///             behaviour.
    fn handle_event_begin(&mut self, _world: &mut World, _event: &mut Event) -> Option<Trans> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `handle_event(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn handle_event_end(&mut self, _world: &mut World, _state_trans: &Trans) -> Option<Trans> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `fixed_update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    fn fixed_update_begin(&mut self, _world: &mut World) -> Option<Trans> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `fixed_update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn fixed_update_end(&mut self, _world: &mut World, _state_trans: &Trans) -> Option<Trans> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked before the delegate state's `update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    fn update_begin(&mut self, _world: &mut World) -> Option<Trans> {
        None
    }
    /// Optionally returns a `Trans` to override the delegate state behaviour.
    ///
    /// Invoked after the delegate state's `update(..)` invocation.
    ///
    /// # Parameters:
    ///
    /// * `world`: The ECS `World`.
    /// * `state_trans`: `Trans` that was returned by the delegate `State`
    fn update_end(&mut self, _world: &mut World, _state_trans: &Trans) -> Option<Trans> {
        None
    }
}
// kcov-ignore-end

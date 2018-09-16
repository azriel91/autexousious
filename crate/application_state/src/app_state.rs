use amethyst::prelude::*;
use application_event::AppEvent;

/// Trait to ease implementation of an amethyst `State`.
///
/// TODO: Ideally we could write `trait AppState<'a, 'b> = State<GameData<'a, 'b>, AppEvent>`, but
/// this is pending RFC 1733. See:
///
/// * <https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md>
/// * <https://github.com/rust-lang/rust/issues/41517>
pub trait AppState<'a, 'b> {
    /// Executed when the game state begins.
    #[inline]
    fn on_start(&mut self, _data: StateData<GameData<'a, 'b>>) {}

    /// Executed when the game state exits.
    #[inline]
    fn on_stop(&mut self, _data: StateData<GameData<'a, 'b>>) {}

    /// Executed when a different game state is pushed onto the stack.
    #[inline]
    fn on_pause(&mut self, _data: StateData<GameData<'a, 'b>>) {}

    /// Executed when the application returns to this game state once again.
    #[inline]
    fn on_resume(&mut self, _data: StateData<GameData<'a, 'b>>) {}

    /// Executed on every frame before updating, for use in reacting to events.
    #[inline]
    fn handle_event(
        &mut self,
        _data: StateData<GameData<'a, 'b>>,
        _event: StateEvent<AppEvent>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    #[inline]
    fn fixed_update(
        &mut self,
        _data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account
    /// the frame rate limit).
    #[inline]
    fn update(&mut self, _data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, AppEvent> {
        Trans::None
    }
}

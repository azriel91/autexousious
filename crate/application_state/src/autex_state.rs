use amethyst::prelude::*;
use application_event::AppEvent;

/// Trait to make `amethyst::State` implementations more readable.
///
/// TODO: Ideally we could write `trait AutexState<'a, 'b> = State<GameData<'a,
/// 'b>, AppEvent>`, but this is pending RFC 1733. See:
///
/// * <https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md>
/// * <https://github.com/rust-lang/rust/issues/41517>
pub trait AutexState<'a, 'b>: State<GameData<'a, 'b>, AppEvent> {}

// Blanket impl.
impl<'a, 'b, T> AutexState<'a, 'b> for T where T: State<GameData<'a, 'b>, AppEvent> {}

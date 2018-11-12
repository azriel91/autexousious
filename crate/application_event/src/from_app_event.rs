use character_selection_model::CharacterSelectionEvent;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use map_selection_model::MapSelectionEvent;

use AppEvent;

/// Trait to obtain the state specific event contained in an `AppEvent`, or the app event.
///
/// TODO: Ideally we can use the [`TryFrom`][try_from] trait, but it's not yet stable, pending
/// <https://github.com/rust-lang/rust/issues/33417>.
///
/// [try_from]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
pub trait FromAppEvent {
    /// Attempts to get an instance of `Self` from an `AppEvent`.
    ///
    /// If this fails, it returns the `AppEvent` itself.
    fn from(app_event: AppEvent) -> Result<Self, AppEvent>
    where
        Self: Sized;
}

macro_rules! impl_from_app_event {
    ($variant:ident, $state_specific_event:ty) => {
        impl FromAppEvent for $state_specific_event {
            /// Returns the state specific event contained in this `AppEvent`, or the app event.
            ///
            /// Ideally we can use the [`TryFrom`][try_from] trait, but it's not yet stable, pending:
            /// <https://github.com/rust-lang/rust/issues/33417>
            ///
            /// [try_from]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
            fn from(app_event: AppEvent) -> Result<$state_specific_event, AppEvent> {
                match app_event {
                    AppEvent::$variant(sse) => Ok(sse),
                    e => Err(e),
                }
            }
        }
    };
}

impl_from_app_event!(CharacterSelection, CharacterSelectionEvent);
impl_from_app_event!(GameModeSelection, GameModeSelectionEvent);
impl_from_app_event!(GamePlay, GamePlayEvent);
impl_from_app_event!(MapSelection, MapSelectionEvent);

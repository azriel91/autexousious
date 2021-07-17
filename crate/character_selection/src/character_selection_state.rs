use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    ecs::{World, WorldExt},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder, AutexState};
use asset_selection_model::play::AssetSelectionEvent;
use character_selection_model::{
    CharacterSelectionEntity, CharacterSelections, CharacterSelectionsStatus,
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use state_registry::StateId;

/// `State` where character selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`CharacterSelectionStateBuilder`][state_builder].
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection
///   is complete.
/// * `S`: State to return.
///
/// [state_builder]:
/// character_selection_state/struct.CharacterSelectionStateBuilder.html
pub type CharacterSelectionState<'a, 'b, F, S> =
    AppState<'a, 'b, CharacterSelectionStateDelegate<'a, 'b, F, S>, CharacterSelectionEntity>;

/// Builder for a `CharacterSelectionState`.
///
/// `SystemBundle`s to run in the `CharacterSelectionState`'s dispatcher are
/// registered on this builder.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection
///   is complete.
/// * `S`: `State` to delegate to.
pub type CharacterSelectionStateBuilder<'a, 'b, F, S> = AppStateBuilder<
    'a,
    'b,
    CharacterSelectionStateDelegate<'a, 'b, F, S>,
    CharacterSelectionEntity,
>;

/// Delegate `State` for character selection.
///
/// This state is not intended to be used directly, but wrapped in an
/// `AppState`. The `CharacterSelectionState` is an alias with this as a
/// delegate state.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection
///   is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct CharacterSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: F,
    /// `PhantomData`.
    marker: PhantomData<dyn AutexState<'a, 'b>>,
}

impl<'a, 'b, F, S> CharacterSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn initialize_character_selections(&mut self, world: &mut World) {
        world.insert(CharacterSelectionsStatus::Waiting);
        world.insert(CharacterSelections::default());
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent>
    for CharacterSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::CharacterSelection);

        self.initialize_character_selections(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::CharacterSelection);

        // TODO: Initialize in the "confirmed" state, including widgets.
        self.initialize_character_selections(&mut data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        if let AppEvent::AssetSelection(asset_selection_event) = event {
            debug!(
                "Received asset_selection_event: {:?}",
                asset_selection_event
            );
            match asset_selection_event {
                AssetSelectionEvent::Return => Trans::Pop,
                AssetSelectionEvent::Confirm => {
                    let character_selections = data.world.read_resource::<CharacterSelections>();
                    debug!(
                        "character_selections: `{:?}`",
                        &character_selections.selections
                    );

                    Trans::Push((self.next_state_fn)())
                }
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    ecs::{World, WorldExt},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder, AutexState};
use asset_model::play::AssetSelectionEvent;
use derivative::Derivative;
use derive_new::new;
use log::debug;
use map_selection_model::{MapSelection, MapSelectionEntity};
use state_registry::StateId;

use crate::MapSelectionStatus;

/// `State` where map selection takes place.
///
/// This state is not intended to be constructed directly, but through the
/// [`MapSelectionStateBuilder`][state_builder].
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after map selection is complete.
/// * `S`: State to return.
///
/// [state_builder]: map_selection_state/struct.MapSelectionStateBuilder.html
pub type MapSelectionState<'a, 'b, F, S> =
    AppState<'a, 'b, MapSelectionStateDelegate<'a, 'b, F, S>, MapSelectionEntity>;

/// Builder for a `MapSelectionState`.
///
/// `SystemBundle`s to run in the `MapSelectionState`'s dispatcher are registered on this
/// builder.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after map selection is complete.
/// * `S`: `State` to delegate to.
pub type MapSelectionStateBuilder<'a, 'b, F, S> =
    AppStateBuilder<'a, 'b, MapSelectionStateDelegate<'a, 'b, F, S>, MapSelectionEntity>;

/// Delegate `State` for map selection.
///
/// This state is not intended to be used directly, but wrapped in an `AppState`. The
/// `MapSelectionState` is an alias with this as a delegate state.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after map selection is complete.
/// * `S`: State to return.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct MapSelectionStateDelegate<'a, 'b, F, S>
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

impl<'a, 'b, F, S> MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn reset_map_selection_state(&self, world: &mut World) {
        world.insert(MapSelectionStatus::Pending);
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::MapSelection);

        self.reset_map_selection_state(&mut data.world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::MapSelection);

        self.reset_map_selection_state(data.world);
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
                    let map_selection = data.world.read_resource::<MapSelection>();

                    // Hack: `AssetSelectionEvent`s are carried through from the previous state
                    // when multiple controllers send input at the same time.
                    if map_selection.asset_id().is_some() {
                        debug!("map_selection: `{:?}`", &*map_selection);

                        Trans::Switch((self.next_state_fn)())
                    } else {
                        debug!("Ignoring asset selection event: {:?}", event);
                        Trans::None
                    }
                }
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

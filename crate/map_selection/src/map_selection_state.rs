use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::{ecs::prelude::*, prelude::*, shrev::EventChannel};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder, AutexState};
use map_selection_model::{MapSelection, MapSelectionEntityId, MapSelectionEvent};

use MapSelectionStatus;

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
    AppState<'a, 'b, MapSelectionStateDelegate<'a, 'b, F, S>, MapSelectionEntityId>;

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
    AppStateBuilder<'a, 'b, MapSelectionStateDelegate<'a, 'b, F, S>, MapSelectionEntityId>;

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
    marker: PhantomData<AutexState<'a, 'b>>,
}

impl<'a, 'b, F, S> MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn reset_map_selection_state(&self, world: &mut World) {
        let mut map_selection_status = world.write_resource::<MapSelectionStatus>();
        *map_selection_status = MapSelectionStatus::Pending;
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.reset_map_selection_state(&mut data.world);
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.reset_map_selection_state(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
        event: StateEvent<AppEvent>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        match event {
            StateEvent::Custom(app_event) => match app_event {
                AppEvent::MapSelection(map_selection_event) => {
                    debug!("Received map_selection_event: {:?}", map_selection_event);
                    let mut channel = data
                        .world
                        .write_resource::<EventChannel<MapSelectionEvent>>();
                    channel.single_write(map_selection_event);
                }
                _ => {}
            },
            _ => {}
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, AppEvent> {
        let map_selection_status = data.world.read_resource::<MapSelectionStatus>();
        if *map_selection_status == MapSelectionStatus::Confirmed {
            let map_selection = data.world.read_resource::<MapSelection>();

            info!("Map selection: `{}`", *map_selection);

            // TODO: `Trans:Push` when we have a proper map selection menu.
            Trans::Switch((self.next_state_fn)())
        } else {
            Trans::None
        }
    }
}

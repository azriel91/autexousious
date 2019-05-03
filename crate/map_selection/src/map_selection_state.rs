use std::{fmt::Debug, marker::PhantomData};

use amethyst::{ecs::prelude::*, prelude::*, shrev::EventChannel};
use application_event::AppEvent;
use application_state::{AppState, AppStateBuilder, AutexState};
use derivative::Derivative;
use derive_new::new;
use log::{debug, info};
use map_selection_model::{MapSelection, MapSelectionEntityId, MapSelectionEvent};
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
    /// Reader ID for the `MapSelectionEvent` event channel.
    #[new(default)]
    map_selection_event_rid: Option<ReaderId<MapSelectionEvent>>,
    /// `PhantomData`.
    marker: PhantomData<dyn AutexState<'a, 'b>>,
}

impl<'a, 'b, F, S> MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn reset_map_selection_state(&self, world: &mut World) {
        world.add_resource(MapSelectionStatus::Pending);
    }

    fn initialize_map_selection_event_rid(&mut self, world: &mut World) {
        let mut map_selection_ec = world.write_resource::<EventChannel<MapSelectionEvent>>();
        self.map_selection_event_rid = Some(map_selection_ec.register_reader());
    }
}

impl<'a, 'b, F, S> State<GameData<'a, 'b>, AppEvent> for MapSelectionStateDelegate<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'b>>) {
        data.world.add_resource(StateId::MapSelection);

        self.reset_map_selection_state(&mut data.world);
        self.initialize_map_selection_event_rid(&mut data.world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.add_resource(StateId::GameModeSelection);

        self.reset_map_selection_state(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        event: AppEvent,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        if let AppEvent::MapSelection(map_selection_event) = event {
            debug!("Received map_selection_event: {:?}", map_selection_event);
            let mut channel = data
                .world
                .write_resource::<EventChannel<MapSelectionEvent>>();
            channel.single_write(map_selection_event);
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        let map_selection_ec = data
            .world
            .read_resource::<EventChannel<MapSelectionEvent>>();
        map_selection_ec
            .read(
                self.map_selection_event_rid
                    .as_mut()
                    .expect("Expected `map_selection_event_rid` to be set."),
            )
            .filter_map(|ev| match ev {
                MapSelectionEvent::Return => Some(Trans::Pop),
                MapSelectionEvent::Confirm => {
                    let map_selection = data.world.read_resource::<MapSelection>();
                    info!("map_selection: `{:?}`", &*map_selection);

                    Some(Trans::Switch((self.next_state_fn)()))
                }
                _ => None,
            })
            .next()
            .unwrap_or_else(|| Trans::None)
    }
}

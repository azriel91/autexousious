use amethyst::{
    ecs::{Join, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use log::debug;
use map_selection_model::{MapSelection, MapSelectionEvent};

use crate::MapSelectionWidgetState;

/// System that processes controller input and generates `MapSelectionEvent`s.
///
/// This is not private because consumers may use `any::type_name::<MapSelectionWidgetInputSystem>()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, new)]
pub struct MapSelectionWidgetInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionWidgetInputResources<'s> {
    /// `MapSelection` components.
    #[derivative(Debug = "ignore")]
    pub map_selections: WriteStorage<'s, MapSelection>,
    /// `MapSelectionWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub map_selection_widget_states: WriteStorage<'s, MapSelectionWidgetState>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `MapSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_selection_ec: Write<'s, EventChannel<MapSelectionEvent>>,
}

/// `MapSelectionWidgetInputSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionWidgetInputSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `MapSelectionWidgetInputResources`.
    pub map_selection_widget_input_resources: MapSelectionWidgetInputResources<'s>,
}

impl MapSelectionWidgetInputSystem {
    fn select_previous_map(
        asset_type_mappings: &AssetTypeMappings,
        map_selection: &mut MapSelection,
    ) -> MapSelection {
        let first_map_id = asset_type_mappings
            .iter_ids(&AssetType::Map)
            .copied()
            .next()
            .expect("Expected at least one map to be loaded.");
        let last_map_id = asset_type_mappings
            .iter_ids(&AssetType::Map)
            .copied()
            .next_back()
            .expect("Expected at least one map to be loaded.");
        *map_selection = match *map_selection {
            MapSelection::Id(map_id) => {
                if map_id == first_map_id {
                    MapSelection::Random(Some(first_map_id))
                } else {
                    let next_map = asset_type_mappings
                        .iter_ids(&AssetType::Map)
                        .copied()
                        .rev()
                        .skip_while(|id| id != &map_id)
                        .nth(1); // skip current selection

                    if let Some(next_map) = next_map {
                        MapSelection::Id(next_map)
                    } else {
                        MapSelection::Random(Some(first_map_id))
                    }
                }
            }
            MapSelection::None | MapSelection::Random(..) => MapSelection::Id(last_map_id),
        };
        *map_selection
    }

    fn select_next_map(
        asset_type_mappings: &AssetTypeMappings,
        map_selection: &mut MapSelection,
    ) -> MapSelection {
        let first_map_id = asset_type_mappings
            .iter_ids(&AssetType::Map)
            .copied()
            .next()
            .expect("Expected at least one map to be loaded.");
        let last_map_id = asset_type_mappings
            .keys()
            .copied()
            .next_back()
            .expect("Expected at least one map to be loaded.");
        *map_selection = match *map_selection {
            MapSelection::Id(map_id) => {
                if map_id == last_map_id {
                    MapSelection::Random(Some(first_map_id))
                } else {
                    let next_map = asset_type_mappings
                        .iter_ids(&AssetType::Map)
                        .copied()
                        .skip_while(|id| id != &map_id)
                        .nth(1); // skip current selection

                    if let Some(next_map) = next_map {
                        MapSelection::Id(next_map)
                    } else {
                        MapSelection::Random(Some(first_map_id))
                    }
                }
            }
            MapSelection::None | MapSelection::Random(..) => MapSelection::Id(first_map_id),
        };
        *map_selection
    }

    fn handle_event(
        MapSelectionWidgetInputResources {
            map_selections,
            map_selection_widget_states,
            asset_type_mappings,
            map_selection_ec,
        }: &mut MapSelectionWidgetInputResources,
        event: ControlInputEvent,
    ) {
        if let Some((map_selection, map_selection_widget_state)) =
            (map_selections, map_selection_widget_states).join().next()
        {
            match event {
                ControlInputEvent::AxisMoved(axis_move_event_data) => Self::handle_axis_event(
                    &asset_type_mappings,
                    map_selection_ec,
                    map_selection,
                    map_selection_widget_state,
                    axis_move_event_data,
                ),
                ControlInputEvent::ControlActionPress(control_action_event_data) => {
                    Self::handle_control_action_event(
                        map_selection_ec,
                        *map_selection,
                        map_selection_widget_state,
                        control_action_event_data,
                    )
                }
                ControlInputEvent::ControlActionRelease(..) => {}
            }
        }
    }

    fn handle_axis_event(
        asset_type_mappings: &AssetTypeMappings,
        map_selection_ec: &mut EventChannel<MapSelectionEvent>,
        map_selection: &mut MapSelection,
        map_selection_widget_state: &mut MapSelectionWidgetState,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let new_map_selection = match (map_selection_widget_state, axis_move_event_data.axis) {
            (MapSelectionWidgetState::MapSelect, Axis::X) if axis_move_event_data.value < 0. => {
                Some(Self::select_previous_map(
                    asset_type_mappings,
                    map_selection,
                ))
            }
            (MapSelectionWidgetState::MapSelect, Axis::X) if axis_move_event_data.value > 0. => {
                Some(Self::select_next_map(asset_type_mappings, map_selection))
            }
            _ => None,
        };

        if let Some(new_map_selection) = new_map_selection {
            let map_selection_event = MapSelectionEvent::Switch {
                map_selection: new_map_selection,
            };

            debug!(
                "Sending map selection event: {:?}",
                &map_selection_event // kcov-ignore
            );
            map_selection_ec.single_write(map_selection_event);
        }
    }

    fn handle_control_action_event(
        map_selection_ec: &mut EventChannel<MapSelectionEvent>,
        map_selection: MapSelection,
        map_selection_widget_state: &mut MapSelectionWidgetState,
        control_action_event_data: ControlActionEventData,
    ) {
        let map_selection_event = match (
            *map_selection_widget_state,
            control_action_event_data.control_action,
        ) {
            (MapSelectionWidgetState::MapSelect, ControlAction::Jump) => {
                Some(MapSelectionEvent::Return)
            }
            (MapSelectionWidgetState::MapSelect, ControlAction::Attack) => {
                *map_selection_widget_state = MapSelectionWidgetState::Ready;
                Some(MapSelectionEvent::Select { map_selection })
            }
            (MapSelectionWidgetState::Ready, ControlAction::Jump) => {
                *map_selection_widget_state = MapSelectionWidgetState::MapSelect;
                Some(MapSelectionEvent::Deselect)
            }
            (MapSelectionWidgetState::Ready, ControlAction::Attack) => {
                Some(MapSelectionEvent::Confirm)
            }
            _ => None,
        };

        if let Some(map_selection_event) = map_selection_event {
            debug!(
                "Sending map selection event: {:?}",
                &map_selection_event // kcov-ignore
            );
            map_selection_ec.single_write(map_selection_event);
        }
    }
}

impl<'s> System<'s> for MapSelectionWidgetInputSystem {
    type SystemData = MapSelectionWidgetInputSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionWidgetInputSystemData {
            control_input_ec,
            mut map_selection_widget_input_resources,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                Self::handle_event(&mut map_selection_widget_input_resources, *ev);
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

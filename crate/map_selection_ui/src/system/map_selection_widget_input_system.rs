use amethyst::{
    ecs::{prelude::*, World},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use asset_model::loaded::SlugAndHandle;
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use game_model::loaded::MapPrefabs;
use log::debug;
use map_selection_model::{MapSelection, MapSelectionEvent};
use typename_derive::TypeName;

use crate::{MapSelectionWidget, WidgetState};

/// System that processes controller input and generates `MapSelectionEvent`s.
///
/// This is not private because consumers may use `MapSelectionWidgetInputSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionWidgetInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub(crate) struct MapSelectionWidgetInputResources<'s> {
    /// `MapSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub map_selection_widgets: WriteStorage<'s, MapSelectionWidget>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub map_prefabs: Read<'s, MapPrefabs>,
    /// `MapSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_selection_ec: Write<'s, EventChannel<MapSelectionEvent>>,
}

type MapSelectionWidgetInputSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    MapSelectionWidgetInputResources<'s>,
);

impl MapSelectionWidgetInputSystem {
    fn select_previous_map(
        map_prefabs: &MapPrefabs,
        widget: &mut MapSelectionWidget,
    ) -> MapSelection {
        let (first_map_slug, first_map_handle) = map_prefabs
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.");
        let (last_map_slug, last_map_handle) = map_prefabs
            .iter()
            .next_back()
            .expect("Expected at least one map to be loaded.");
        widget.selection = match widget.selection {
            MapSelection::Id(SlugAndHandle {
                slug: ref map_slug, ..
            }) => {
                if map_slug == first_map_slug {
                    MapSelection::Random((first_map_slug, first_map_handle).into())
                } else {
                    let next_map = map_prefabs
                        .iter()
                        .rev()
                        .skip_while(|(slug, _handle)| slug != &map_slug)
                        .nth(1); // skip current selection

                    if let Some(next_map) = next_map {
                        MapSelection::Id(next_map.into())
                    } else {
                        MapSelection::Random((first_map_slug, first_map_handle).into())
                    }
                }
            }
            MapSelection::Random(..) => MapSelection::Id((last_map_slug, last_map_handle).into()),
        };
        widget.selection.clone()
    }

    fn select_next_map(map_prefabs: &MapPrefabs, widget: &mut MapSelectionWidget) -> MapSelection {
        let (first_map_slug, first_map_handle) = map_prefabs
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.");
        let last_map_slug = map_prefabs
            .keys()
            .next_back()
            .expect("Expected at least one map to be loaded.");
        widget.selection = match widget.selection {
            MapSelection::Id(SlugAndHandle {
                slug: ref map_slug, ..
            }) => {
                if map_slug == last_map_slug {
                    MapSelection::Random((first_map_slug, first_map_handle).into())
                } else {
                    let next_map = map_prefabs
                        .iter()
                        .skip_while(|(slug, _handle)| slug != &map_slug)
                        .nth(1); // skip current selection

                    if let Some(next_map) = next_map {
                        MapSelection::Id(next_map.into())
                    } else {
                        MapSelection::Random((first_map_slug, first_map_handle).into())
                    }
                }
            }
            MapSelection::Random(..) => MapSelection::Id((first_map_slug, first_map_handle).into()),
        };
        widget.selection.clone()
    }

    fn handle_event(
        MapSelectionWidgetInputResources {
            ref mut map_selection_widgets,
            ref map_prefabs,
            ref mut map_selection_ec,
        }: &mut MapSelectionWidgetInputResources,
        event: ControlInputEvent,
    ) {
        if let Some(map_selection_widget) = map_selection_widgets.join().next() {
            match event {
                ControlInputEvent::AxisMoved(axis_move_event_data) => Self::handle_axis_event(
                    &map_prefabs,
                    map_selection_ec,
                    map_selection_widget,
                    axis_move_event_data,
                ),
                ControlInputEvent::ControlActionPress(control_action_event_data) => {
                    Self::handle_control_action_event(
                        map_selection_ec,
                        map_selection_widget,
                        control_action_event_data,
                    )
                }
                ControlInputEvent::ControlActionRelease(..) => {}
            }
        }
    }

    fn handle_axis_event(
        map_prefabs: &MapPrefabs,
        map_selection_ec: &mut EventChannel<MapSelectionEvent>,
        map_selection_widget: &mut MapSelectionWidget,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let map_selection = match (map_selection_widget.state, axis_move_event_data.axis) {
            (WidgetState::MapSelect, Axis::X) if axis_move_event_data.value < 0. => {
                Some(Self::select_previous_map(map_prefabs, map_selection_widget))
            }
            (WidgetState::MapSelect, Axis::X) if axis_move_event_data.value > 0. => {
                Some(Self::select_next_map(map_prefabs, map_selection_widget))
            }
            _ => None,
        };

        if let Some(map_selection) = map_selection {
            let map_selection_event = MapSelectionEvent::Switch { map_selection };

            debug!(
                "Sending map selection event: {:?}",
                &map_selection_event // kcov-ignore
            );
            map_selection_ec.single_write(map_selection_event);
        }
    }

    fn handle_control_action_event(
        map_selection_ec: &mut EventChannel<MapSelectionEvent>,
        map_selection_widget: &mut MapSelectionWidget,
        control_action_event_data: ControlActionEventData,
    ) {
        let map_selection_event = match (
            map_selection_widget.state,
            control_action_event_data.control_action,
        ) {
            (WidgetState::MapSelect, ControlAction::Jump) => Some(MapSelectionEvent::Return),
            (WidgetState::MapSelect, ControlAction::Attack) => {
                map_selection_widget.state = WidgetState::Ready;
                Some(MapSelectionEvent::Select {
                    map_selection: map_selection_widget.selection.clone(),
                })
            }
            (WidgetState::Ready, ControlAction::Jump) => {
                map_selection_widget.state = WidgetState::MapSelect;
                Some(MapSelectionEvent::Deselect)
            }
            (WidgetState::Ready, ControlAction::Attack) => Some(MapSelectionEvent::Confirm),
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
        (control_input_ec, mut map_selection_widget_input_resources): Self::SystemData,
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

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.control_input_event_rid = Some(
            res.fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, SystemData, World},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::MAP_FADE_SLUG;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use game_model::loaded::MapPrefabs;
    use map_model::loaded::Map;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use typename::TypeName;

    use super::{MapSelectionWidgetInputSystem, MapSelectionWidgetInputSystemData};
    use crate::{MapSelectionWidget, WidgetState};

    #[test]
    fn does_not_send_event_when_no_input() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: None,
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                map_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn selects_last_map_when_input_left_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: Some(press_left),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let last_map = last_map(world);
                    MapSelection::Id(last_map)
                },
                map_selection_events_fn: |world| {
                    let last_map = last_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Id(last_map),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_first_map_when_input_right_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let first_map = first_map(world);
                    MapSelection::Id(first_map)
                },
                map_selection_events_fn: |world| {
                    let first_map = first_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Id(first_map),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_random_when_input_right_and_selection_last_map() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let last_map = last_map(world);
                    MapSelection::Id(last_map)
                },
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                map_selection_events_fn: |world| {
                    let first_map = first_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Random(first_map),
                    }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_map_select_to_ready_and_sends_event_when_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |world| {
                    vec![MapSelectionEvent::Select {
                        map_selection: map_selection_fade(world),
                    }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_ready_to_map_select_and_sends_event_when_input_jump() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Deselect],
            },
        )
    }

    #[test]
    fn sends_confirm_event_when_widget_ready_and_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Confirm],
            },
        )
    }

    #[test]
    fn send_return_event_when_controller_input_jump_and_widget_map_select() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Return],
            },
        )
    }

    fn run_test(
        SetupParams {
            widget_state: widget_entity_state,
            map_selection_fn: setup_map_selection_fn,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            widget_state: expected_widget_state,
            map_selection_fn: expected_map_selection_fn,
            map_selection_events_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(move |world| {
                MapSelectionWidgetInputSystemData::setup(&mut world.res);

                // Setup event reader.
                let event_channel_reader = world
                    .write_resource::<EventChannel<MapSelectionEvent>>()
                    .register_reader(); // kcov-ignore
                world.insert(event_channel_reader);

                let map_selection = setup_map_selection_fn(world);
                let entity = widget_entity(world, widget_entity_state, map_selection);
                world.insert(entity);
            })
            .with_effect(move |world| {
                if let Some(control_input_event_fn) = control_input_event_fn {
                    let entity = *world.read_resource::<Entity>();
                    world
                        .write_resource::<EventChannel<ControlInputEvent>>()
                        .single_write(control_input_event_fn(entity));
                }
            })
            .with_assertion(move |world| {
                let expected_map_selection = expected_map_selection_fn(world);
                assert_widget(
                    world,
                    MapSelectionWidget::new(expected_widget_state, expected_map_selection),
                )
            })
            .with_assertion(move |world| {
                let map_selection_events = map_selection_events_fn(world);
                assert_events(world, map_selection_events);
            })
            .run_isolated()
    }

    fn map_selection_fade(world: &mut World) -> MapSelection {
        let fade_map = fade_map(world);
        MapSelection::Id(fade_map)
    }

    fn map_selection_random(world: &mut World) -> MapSelection {
        let first_map = first_map(world);
        MapSelection::Random(first_map)
    }

    fn press_left(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::X,
            value: -1.,
        })
    }

    fn press_right(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::X,
            value: 1.,
        })
    }

    fn press_jump(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Jump,
        })
    }

    fn press_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Attack,
        })
    }

    fn empty_events(_world: &mut World) -> Vec<MapSelectionEvent> {
        vec![]
    }

    fn first_map(world: &mut World) -> SlugAndHandle<Map> {
        world
            .read_resource::<MapPrefabs>()
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn last_map(world: &mut World) -> SlugAndHandle<Map> {
        world
            .read_resource::<MapPrefabs>()
            .iter()
            .next_back()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn fade_map(world: &mut World) -> SlugAndHandle<Map> {
        SlugAndHandle::from((&*world, MAP_FADE_SLUG.clone()))
    }

    fn widget_entity(
        world: &mut World,
        widget_state: WidgetState,
        map_selection: MapSelection,
    ) -> Entity {
        world
            .create_entity()
            .with(MapSelectionWidget::new(widget_state, map_selection))
            .build()
    }

    fn assert_widget(world: &mut World, expected: MapSelectionWidget) {
        let widget_entity = world.read_resource::<Entity>();

        let widgets = world.read_storage::<MapSelectionWidget>();
        let widget = widgets
            .get(*widget_entity)
            .expect("Expected entity to have `MapSelectionWidget` component.");

        assert_eq!(expected, *widget);
    }

    fn assert_events(world: &mut World, events: Vec<MapSelectionEvent>) {
        let mut event_channel_reader = &mut world.write_resource::<ReaderId<MapSelectionEvent>>();

        let map_selection_event_channel = world.read_resource::<EventChannel<MapSelectionEvent>>();
        let actual_events = map_selection_event_channel
            .read(&mut event_channel_reader)
            .collect::<Vec<&MapSelectionEvent>>();

        let expected_events = events.iter().collect::<Vec<&MapSelectionEvent>>();
        assert_eq!(expected_events, actual_events);
    }

    struct SetupParams {
        widget_state: WidgetState,
        map_selection_fn: fn(&mut World) -> MapSelection,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams {
        widget_state: WidgetState,
        map_selection_fn: fn(&mut World) -> MapSelection,
        map_selection_events_fn: fn(&mut World) -> Vec<MapSelectionEvent>,
    }
}

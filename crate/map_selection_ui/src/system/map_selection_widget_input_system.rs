use amethyst::{ecs::prelude::*, shrev::EventChannel};
use game_input::ControllerInput;
use game_model::loaded::{MapAssets, SlugAndHandle};
use map_selection_model::{MapSelection, MapSelectionEvent};
use tracker::Last;

use MapSelectionWidget;
use WidgetState;

/// System that processes controller input and generates `MapSelectionEvent`s.
///
/// This is not private because consumers may use `MapSelectionWidgetInputSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionWidgetInputSystem;

type MapSelectionWidgetInputSystemData<'s> = (
    Read<'s, MapAssets>,
    WriteStorage<'s, MapSelectionWidget>,
    ReadStorage<'s, Last<ControllerInput>>,
    ReadStorage<'s, ControllerInput>,
    Write<'s, EventChannel<MapSelectionEvent>>,
);

impl MapSelectionWidgetInputSystem {
    fn handle_map_select(
        map_assets: &MapAssets,
        widget: &mut MapSelectionWidget,
        last_input: &Last<ControllerInput>,
        input: &ControllerInput,
        event_channel: &mut EventChannel<MapSelectionEvent>,
    ) {
        if !last_input.attack && input.attack {
            widget.state = WidgetState::Ready;

            // Send map selection event
            let map_selection_event = MapSelectionEvent::Select {
                map_selection: widget.selection.clone(),
            };
            debug!("Sending map selection event: {:?}", &map_selection_event);
            event_channel.single_write(map_selection_event);
        } else if last_input.x_axis_value == 0. && input.x_axis_value < 0. {
            Self::select_previous_map(map_assets, widget);
        } else if last_input.x_axis_value == 0. && input.x_axis_value > 0. {
            Self::select_next_map(map_assets, widget);
        }
    }

    fn select_previous_map(map_assets: &MapAssets, widget: &mut MapSelectionWidget) {
        let (first_map_slug, first_map_handle) = map_assets
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.");
        let (last_map_slug, last_map_handle) = map_assets
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
                    let next_map = map_assets
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
    }

    fn select_next_map(map_assets: &MapAssets, widget: &mut MapSelectionWidget) {
        let (first_map_slug, first_map_handle) = map_assets
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.");
        let last_map_slug = map_assets
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
                    let next_map = map_assets
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
    }
}

impl<'s> System<'s> for MapSelectionWidgetInputSystem {
    type SystemData = MapSelectionWidgetInputSystemData<'s>;

    fn run(
        &mut self,
        (
            map_assets,
            mut map_selection_widgets,
            last_controller_inputs,
            controller_inputs,
            mut map_selection_events,
        ): Self::SystemData,
    ) {
        for (mut widget, last_input, input) in (
            &mut map_selection_widgets,
            &last_controller_inputs,
            &controller_inputs,
        )
            .join()
        {
            if let WidgetState::MapSelect = widget.state {
                Self::handle_map_select(
                    &map_assets,
                    &mut widget,
                    &last_input,
                    &input,
                    &mut map_selection_events,
                )
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::prelude::*,
        shrev::{EventChannel, ReaderId},
    };
    use amethyst_test_support::prelude::*;
    use application_test_support::AutexousiousApplication;
    use assets_test::ASSETS_MAP_EMPTY_SLUG;
    use game_input::ControllerInput;
    use game_model::loaded::{MapAssets, SlugAndHandle};
    use map_model::loaded::Map;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use tracker::Last;
    use typename::TypeName;

    use super::{MapSelectionWidgetInputSystem, MapSelectionWidgetInputSystemData};
    use MapSelectionWidget;
    use WidgetState;

    #[test]
    fn does_not_send_event_when_controller_input_empty() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "does_not_send_event_when_controller_input_empty",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(|world| {
                let empty_snh = SlugAndHandle::from((&*world, ASSETS_MAP_EMPTY_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::MapSelect,
                    MapSelection::Id(empty_snh),
                    ControllerInput::default(),
                )
            }).with_system_single(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_map_select_to_ready_and_sends_event_when_input_attack() {
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_map_select_to_ready_and_sends_event_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let empty_snh = SlugAndHandle::from((&*world, ASSETS_MAP_EMPTY_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::MapSelect,
                    MapSelection::Id(empty_snh),
                    controller_input,
                )
            }).with_system_single(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let empty_snh = SlugAndHandle::from((&*world, ASSETS_MAP_EMPTY_SLUG.clone()));
                assert_widget(
                    world,
                    MapSelectionWidget::new(WidgetState::Ready, MapSelection::Id(empty_snh)),
                )
            }).with_assertion(|world| {
                let empty_snh = SlugAndHandle::from((&*world, ASSETS_MAP_EMPTY_SLUG.clone()));
                assert_events(
                    world,
                    vec![MapSelectionEvent::Select {
                        map_selection: MapSelection::Id(empty_snh),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_last_map_when_input_left_and_selection_random() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_last_map_when_input_left_and_selection_random",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let first_snh = first_map(world);
                setup_widget(
                    world,
                    WidgetState::MapSelect,
                    MapSelection::Random(first_snh),
                    controller_input,
                )
            }).with_system_single(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let last_snh = last_map(world);
                assert_widget(
                    world,
                    MapSelectionWidget::new(WidgetState::MapSelect, MapSelection::Id(last_snh)),
                )
            }).with_assertion(|world| {
                let last_snh = last_map(world);
                assert_events(
                    world,
                    vec![MapSelectionEvent::Select {
                        map_selection: MapSelection::Id(last_snh),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_first_map_when_input_right_and_selection_random() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_first_map_when_input_right_and_selection_random",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let first_snh = first_map(world);
                setup_widget(
                    world,
                    WidgetState::MapSelect,
                    MapSelection::Random(first_snh),
                    controller_input,
                )
            }).with_system_single(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let first_snh = first_map(world);
                assert_widget(
                    world,
                    MapSelectionWidget::new(WidgetState::MapSelect, MapSelection::Id(first_snh)),
                )
            }).with_assertion(|world| {
                let first_snh = first_map(world);
                assert_events(
                    world,
                    vec![MapSelectionEvent::Select {
                        map_selection: MapSelection::Id(first_snh),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_random_when_input_right_and_selection_last_map() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_random_when_input_right_and_selection_last_map",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let last_snh = last_map(world);
                setup_widget(
                    world,
                    WidgetState::MapSelect,
                    MapSelection::Id(last_snh),
                    controller_input,
                )
            }).with_system_single(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let first_snh = first_map(world);
                assert_widget(
                    world,
                    MapSelectionWidget::new(
                        WidgetState::MapSelect,
                        MapSelection::Random(first_snh),
                    ),
                )
            }).with_assertion(|world| {
                let empty_snh = SlugAndHandle::from((&*world, ASSETS_MAP_EMPTY_SLUG.clone()));
                assert_events(
                    world,
                    vec![MapSelectionEvent::Select {
                        map_selection: MapSelection::Id(empty_snh),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    fn first_map(world: &mut World) -> SlugAndHandle<Map> {
        world
            .read_resource::<MapAssets>()
            .iter()
            .next()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn last_map(world: &mut World) -> SlugAndHandle<Map> {
        world
            .read_resource::<MapAssets>()
            .iter()
            .next_back()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn setup_components(world: &mut World) {
        MapSelectionWidgetInputSystemData::setup(&mut world.res);
    }

    fn setup_event_reader(world: &mut World) {
        let event_channel_reader = world
            .write_resource::<EventChannel<MapSelectionEvent>>()
            .register_reader(); // kcov-ignore

        world.add_resource(EffectReturn(event_channel_reader));
    }

    fn setup_widget(
        world: &mut World,
        widget_state: WidgetState,
        map_selection: MapSelection,
        controller_input: ControllerInput,
    ) {
        let widget = world
            .create_entity()
            .with(MapSelectionWidget::new(widget_state, map_selection))
            .with(controller_input)
            .with(Last(ControllerInput::default()))
            .build();

        world.add_resource(EffectReturn(widget));
    }

    fn assert_widget(world: &mut World, expected: MapSelectionWidget) {
        let widget_entity = &world.read_resource::<EffectReturn<Entity>>().0;

        let widgets = world.read_storage::<MapSelectionWidget>();
        let widget = widgets
            .get(*widget_entity)
            .expect("Expected entity to have `MapSelectionWidget` component.");

        assert_eq!(expected, *widget);
    }

    fn assert_events(world: &mut World, events: Vec<MapSelectionEvent>) {
        let mut event_channel_reader = &mut world
            .write_resource::<EffectReturn<ReaderId<MapSelectionEvent>>>()
            .0;

        let map_selection_event_channel = world.read_resource::<EventChannel<MapSelectionEvent>>();
        let map_selection_event_iter = map_selection_event_channel.read(&mut event_channel_reader);

        let expected_events_iter = events.into_iter();
        expected_events_iter
            .zip(map_selection_event_iter)
            .for_each(|(expected_event, actual)| assert_eq!(expected_event, *actual));
    }
}

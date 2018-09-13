use amethyst::{ecs::prelude::*, shrev::EventChannel};
use character_selection::{CharacterSelection, CharacterSelectionEvent};
use game_input::{ControllerInput, InputControlled};
use game_model::loaded::{CharacterAssets, SlugAndHandle};
use tracker::Last;

use CharacterSelectionWidget;
use WidgetState;

/// System that processes controller input and generates `CharacterSelectionEvent`s.
///
/// This is not private because consumers may use `CharacterSelectionWidgetInputSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionWidgetInputSystem;

type CharacterSelectionWidgetInputSystemData<'s> = (
    Read<'s, CharacterAssets>,
    WriteStorage<'s, CharacterSelectionWidget>,
    ReadStorage<'s, InputControlled>,
    ReadStorage<'s, Last<ControllerInput>>,
    ReadStorage<'s, ControllerInput>,
    Write<'s, EventChannel<CharacterSelectionEvent>>,
);

impl CharacterSelectionWidgetInputSystem {
    fn handle_inactive(
        widget: &mut CharacterSelectionWidget,
        controlled: &InputControlled,
        last_input: &Last<ControllerInput>,
        input: &ControllerInput,
    ) {
        if !last_input.attack && input.attack {
            debug!("Controller {} active.", controlled.controller_id);
            widget.state = WidgetState::CharacterSelect;
        }
    }

    fn handle_character_select(
        character_assets: &CharacterAssets,
        widget: &mut CharacterSelectionWidget,
        controlled: &InputControlled,
        last_input: &Last<ControllerInput>,
        input: &ControllerInput,
        event_channel: &mut EventChannel<CharacterSelectionEvent>,
    ) {
        if !last_input.jump && input.jump {
            debug!("Controller {} inactive.", controlled.controller_id);
            widget.state = WidgetState::Inactive;
        } else if !last_input.attack && input.attack {
            debug!("Controller {} ready.", controlled.controller_id);
            widget.state = WidgetState::Ready;

            // Send character selection event
            let character_selection_event = CharacterSelectionEvent::Select {
                controller_id: controlled.controller_id,
                character_selection: widget.selection.clone(),
            };
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event
            );
            event_channel.single_write(character_selection_event);
        } else if last_input.x_axis_value == 0. && input.x_axis_value < 0. {
            Self::select_previous_character(character_assets, widget);
        } else if last_input.x_axis_value == 0. && input.x_axis_value > 0. {
            Self::select_next_character(character_assets, widget);
        }
    }

    fn handle_ready(
        widget: &mut CharacterSelectionWidget,
        controlled: &InputControlled,
        last_input: &Last<ControllerInput>,
        input: &ControllerInput,
        event_channel: &mut EventChannel<CharacterSelectionEvent>,
    ) {
        if !last_input.jump && input.jump {
            widget.state = WidgetState::CharacterSelect;

            let character_selection_event = CharacterSelectionEvent::Deselect {
                controller_id: controlled.controller_id,
            };
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event
            );
            event_channel.single_write(character_selection_event);
        } else if !last_input.attack && input.attack {
            let character_selection_event = CharacterSelectionEvent::Confirm;
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event
            );
            event_channel.single_write(character_selection_event);
        }
    }

    fn select_previous_character(
        character_assets: &CharacterAssets,
        widget: &mut CharacterSelectionWidget,
    ) {
        let (first_character_slug, first_character_handle) = character_assets
            .iter()
            .next()
            .expect("Expected at least one character to be loaded.");
        let (last_character_slug, last_character_handle) = character_assets
            .iter()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(SlugAndHandle {
                slug: ref character_slug,
                ..
            }) => {
                if character_slug == first_character_slug {
                    CharacterSelection::Random(
                        (first_character_slug, first_character_handle).into(),
                    )
                } else {
                    let next_character = character_assets
                        .iter()
                        .rev()
                        .skip_while(|(slug, _handle)| slug != &character_slug)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character.into())
                    } else {
                        CharacterSelection::Random(
                            (first_character_slug, first_character_handle).into(),
                        )
                    }
                }
            }
            CharacterSelection::Random(..) => {
                CharacterSelection::Id((last_character_slug, last_character_handle).into())
            }
        };
    }

    fn select_next_character(
        character_assets: &CharacterAssets,
        widget: &mut CharacterSelectionWidget,
    ) {
        let (first_character_slug, first_character_handle) = character_assets
            .iter()
            .next()
            .expect("Expected at least one character to be loaded.");
        let last_character_slug = character_assets
            .keys()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(SlugAndHandle {
                slug: ref character_slug,
                ..
            }) => {
                if character_slug == last_character_slug {
                    CharacterSelection::Random(
                        (first_character_slug, first_character_handle).into(),
                    )
                } else {
                    let next_character = character_assets
                        .iter()
                        .skip_while(|(slug, _handle)| slug != &character_slug)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character.into())
                    } else {
                        CharacterSelection::Random(
                            (first_character_slug, first_character_handle).into(),
                        )
                    }
                }
            }
            CharacterSelection::Random(..) => {
                CharacterSelection::Id((first_character_slug, first_character_handle).into())
            }
        };
    }
}

impl<'s> System<'s> for CharacterSelectionWidgetInputSystem {
    type SystemData = CharacterSelectionWidgetInputSystemData<'s>;

    fn run(
        &mut self,
        (
            character_assets,
            mut character_selection_widgets,
            input_controlleds,
            last_controller_inputs,
            controller_inputs,
            mut character_selection_events,
        ): Self::SystemData,
    ) {
        for (mut widget, input_controlled, last_input, input) in (
            &mut character_selection_widgets,
            &input_controlleds,
            &last_controller_inputs,
            &controller_inputs,
        )
            .join()
        {
            match widget.state {
                WidgetState::Inactive => {
                    Self::handle_inactive(&mut widget, &input_controlled, &last_input, &input)
                }
                WidgetState::CharacterSelect => Self::handle_character_select(
                    &character_assets,
                    &mut widget,
                    &input_controlled,
                    &last_input,
                    &input,
                    &mut character_selection_events,
                ),
                WidgetState::Ready => Self::handle_ready(
                    &mut widget,
                    &input_controlled,
                    &last_input,
                    &input,
                    &mut character_selection_events,
                ),
            };
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
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_selection::{CharacterSelection, CharacterSelectionEvent};
    use game_input::{ControllerInput, InputControlled};
    use game_model::loaded::{CharacterAssets, SlugAndHandle};
    use object_model::loaded::Character;
    use tracker::Last;
    use typename::TypeName;

    use super::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetInputSystemData};
    use CharacterSelectionWidget;
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
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::Inactive,
                    CharacterSelection::Id(bat_snh),
                    ControllerInput::default(),
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_inactive_to_character_select_when_input_attack() {
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_inactive_to_character_select_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let first_char = first_character(world);
                setup_widget(
                    world,
                    WidgetState::Inactive,
                    CharacterSelection::Random(first_char),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let first_char = first_character(world);
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::CharacterSelect,
                        CharacterSelection::Random(first_char),
                    ),
                )
            }).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_character_select_to_ready_and_sends_event_when_input_attack() {
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_character_select_to_ready_and_sends_event_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::CharacterSelect,
                    CharacterSelection::Id(bat_snh),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::Ready,
                        CharacterSelection::Id(bat_snh),
                    ),
                )
            }).with_assertion(|world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                assert_events(
                    world,
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(bat_snh),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn sends_confirm_event_when_widget_ready_and_input_attack() {
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_character_select_to_ready_and_sends_event_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::Ready,
                    CharacterSelection::Id(bat_snh),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_events(world, vec![CharacterSelectionEvent::Confirm]))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn selects_last_character_when_input_left_and_selection_random() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_last_character_when_input_left_and_selection_random",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let first_char = first_character(world);
                setup_widget(
                    world,
                    WidgetState::CharacterSelect,
                    CharacterSelection::Random(first_char),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let last_char = last_character(world);
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::CharacterSelect,
                        CharacterSelection::Id(last_char),
                    ),
                )
            }).with_assertion(|world| {
                let last_char = last_character(world);
                assert_events(
                    world,
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(last_char),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_first_character_when_input_right_and_selection_random() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_first_character_when_input_right_and_selection_random",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let first_char = first_character(world);
                setup_widget(
                    world,
                    WidgetState::CharacterSelect,
                    CharacterSelection::Random(first_char),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let first_char = first_character(world);
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::CharacterSelect,
                        CharacterSelection::Id(first_char),
                    ),
                )
            }).with_assertion(|world| {
                let first_char = first_character(world);
                assert_events(
                    world,
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(first_char),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_random_when_input_right_and_selection_last_character() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "selects_random_when_input_right_and_selection_last_character",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::CharacterSelect,
                    CharacterSelection::Id(bat_snh),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let first_char = first_character(world);
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::CharacterSelect,
                        CharacterSelection::Random(first_char),
                    ),
                )
            }).with_assertion(|world| {
                let first_char = first_character(world);
                assert_events(
                    world,
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(first_char),
                    }],
                )
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_ready_to_character_select_and_sends_event_when_input_jump() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_ready_to_character_select_and_sends_event_when_input_jump",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::Ready,
                    CharacterSelection::Id(bat_snh),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::CharacterSelect,
                        CharacterSelection::Id(bat_snh),
                    ),
                )
            }).with_assertion(|world| assert_events(
                world,
                vec![CharacterSelectionEvent::Deselect { controller_id: 123 }]
            )).run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_character_select_to_inactive_when_input_jump() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "updates_widget_character_select_to_inactive_when_input_jump",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                setup_widget(
                    world,
                    WidgetState::CharacterSelect,
                    CharacterSelection::Id(bat_snh),
                    controller_input,
                )
            }).with_system_single(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[]
            ).with_assertion(|world| {
                let bat_snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                assert_widget(
                    world,
                    CharacterSelectionWidget::new(
                        WidgetState::Inactive,
                        CharacterSelection::Id(bat_snh),
                    ),
                )
            }).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    fn first_character(world: &mut World) -> SlugAndHandle<Character> {
        world
            .read_resource::<CharacterAssets>()
            .iter()
            .next()
            .expect("Expected at least one character to be loaded.")
            .into()
    }

    fn last_character(world: &mut World) -> SlugAndHandle<Character> {
        world
            .read_resource::<CharacterAssets>()
            .iter()
            .next_back()
            .expect("Expected at least one character to be loaded.")
            .into()
    }

    fn setup_components(world: &mut World) {
        CharacterSelectionWidgetInputSystemData::setup(&mut world.res);
    }

    fn setup_event_reader(world: &mut World) {
        let event_channel_reader = world
            .write_resource::<EventChannel<CharacterSelectionEvent>>()
            .register_reader(); // kcov-ignore

        world.add_resource(EffectReturn(event_channel_reader));
    }

    fn setup_widget(
        world: &mut World,
        widget_state: WidgetState,
        character_selection: CharacterSelection,
        controller_input: ControllerInput,
    ) {
        let widget = world
            .create_entity()
            .with(CharacterSelectionWidget::new(
                widget_state,
                character_selection,
            )).with(InputControlled::new(123))
            .with(controller_input)
            .with(Last(ControllerInput::default()))
            .build();

        world.add_resource(EffectReturn(widget));
    }

    fn assert_widget(world: &mut World, expected: CharacterSelectionWidget) {
        let widget_entity = &world.read_resource::<EffectReturn<Entity>>().0;

        let widgets = world.read_storage::<CharacterSelectionWidget>();
        let widget = widgets
            .get(*widget_entity)
            .expect("Expected entity to have `CharacterSelectionWidget` component.");

        assert_eq!(expected, *widget);
    }

    fn assert_events(world: &mut World, events: Vec<CharacterSelectionEvent>) {
        let mut event_channel_reader = &mut world
            .write_resource::<EffectReturn<ReaderId<CharacterSelectionEvent>>>()
            .0;

        let character_selection_event_channel =
            world.read_resource::<EventChannel<CharacterSelectionEvent>>();
        let character_selection_event_iter =
            character_selection_event_channel.read(&mut event_channel_reader);

        let expected_events_iter = events.into_iter();
        expected_events_iter
            .zip(character_selection_event_iter)
            .for_each(|(expected_event, actual)| assert_eq!(expected_event, *actual));
    }
}

use amethyst::{ecs::prelude::*, shrev::EventChannel};
use character_selection::{CharacterSelection, CharacterSelectionEvent};
use game_input::{ControllerInput, InputControlled};
use object_model::loaded::CharacterHandle;

use CharacterSelectionWidget;
use WidgetState;

/// System that processes controller input and generates `CharacterSelectionEvent`s.
///
/// This is not private because consumers may use `CharacterSelectionWidgetUiSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionWidgetUiSystem;

type CharacterSelectionWidgetUiSystemData<'s> = (
    Read<'s, Vec<CharacterHandle>>,
    WriteStorage<'s, CharacterSelectionWidget>,
    ReadStorage<'s, InputControlled>,
    ReadStorage<'s, ControllerInput>,
    Write<'s, EventChannel<CharacterSelectionEvent>>,
);

impl CharacterSelectionWidgetUiSystem {
    fn handle_inactive(widget: &mut CharacterSelectionWidget, input: &ControllerInput) {
        if input.attack {
            widget.state = WidgetState::CharacterSelect;
        }
    }

    fn handle_character_select(
        character_handles: &[CharacterHandle],
        widget: &mut CharacterSelectionWidget,
        controlled: &InputControlled,
        input: &ControllerInput,
        event_channel: &mut EventChannel<CharacterSelectionEvent>,
    ) {
        if input.jump {
            widget.state = WidgetState::Inactive;
        } else if input.attack {
            widget.state = WidgetState::Ready;

            // Send character selection event
            let character_selection_event = CharacterSelectionEvent::Select {
                controller_id: controlled.controller_id,
                character_selection: widget.selection,
            };
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event
            );
            event_channel.single_write(character_selection_event);
        } else if input.x_axis_value < 0. {
            Self::select_previous_character(character_handles, widget);
        } else if input.x_axis_value > 0. {
            Self::select_next_character(character_handles, widget);
        }
    }

    fn handle_ready(
        widget: &mut CharacterSelectionWidget,
        controlled: &InputControlled,
        input: &ControllerInput,
        event_channel: &mut EventChannel<CharacterSelectionEvent>,
    ) {
        if input.jump {
            widget.state = WidgetState::CharacterSelect;
            // Send character selection event
            let character_selection_event = CharacterSelectionEvent::Deselect {
                controller_id: controlled.controller_id,
            };
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event
            );
            event_channel.single_write(character_selection_event);
        }
    }

    fn select_previous_character(
        character_handles: &[CharacterHandle],
        widget: &mut CharacterSelectionWidget,
    ) {
        if character_handles.is_empty() {
            panic!("Expected at least one character to be loaded.");
        } else {
            let last_character_index = if character_handles.len() == 1 {
                0
            } else {
                character_handles.len() - 1
            };
            widget.selection = match widget.selection {
                CharacterSelection::Id(0) => CharacterSelection::Random,
                CharacterSelection::Id(id) => CharacterSelection::Id(id - 1),
                CharacterSelection::Random => CharacterSelection::Id(last_character_index),
            };
        }
    }

    fn select_next_character(
        character_handles: &[CharacterHandle],
        widget: &mut CharacterSelectionWidget,
    ) {
        widget.selection = match widget.selection {
            CharacterSelection::Id(id) => {
                let next_index = id + 1;
                if next_index == character_handles.len() {
                    CharacterSelection::Random
                } else {
                    CharacterSelection::Id(next_index)
                }
            }
            CharacterSelection::Random => CharacterSelection::Id(0),
        };
    }
}

impl<'s> System<'s> for CharacterSelectionWidgetUiSystem {
    type SystemData = CharacterSelectionWidgetUiSystemData<'s>;

    fn run(
        &mut self,
        (
            character_handles,
            mut character_selection_widgets,
            input_controlleds,
            controller_inputs,
            mut character_selection_events,
        ): Self::SystemData,
    ) {
        for (mut widget, input_controlled, input) in (
            &mut character_selection_widgets,
            &input_controlleds,
            &controller_inputs,
        )
            .join()
        {
            match widget.state {
                WidgetState::Inactive => Self::handle_inactive(&mut widget, &input),
                WidgetState::CharacterSelect => Self::handle_character_select(
                    &character_handles,
                    &mut widget,
                    &input_controlled,
                    &input,
                    &mut character_selection_events,
                ),
                WidgetState::Ready => Self::handle_ready(
                    &mut widget,
                    &input_controlled,
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
    use character_selection::{CharacterSelection, CharacterSelectionEvent};
    use game_input::{ControllerInput, InputControlled};
    use typename::TypeName;

    use super::{CharacterSelectionWidgetUiSystem, CharacterSelectionWidgetUiSystemData};
    use CharacterSelectionWidget;
    use WidgetState;

    #[test]
    fn does_not_send_event_when_controller_input_empty() {
        assert!(
            AutexousiousApplication::config_base(
                "does_not_send_event_when_controller_input_empty",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(|world| setup_widget(
                world,
                WidgetState::Inactive,
                CharacterSelection::Id(0),
                ControllerInput::default()
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
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

        assert!(
            AutexousiousApplication::config_base(
                "updates_widget_inactive_to_character_select_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| setup_widget(
                world,
                WidgetState::Inactive,
                CharacterSelection::Random,
                controller_input
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_widget(
                world,
                CharacterSelectionWidget::new(
                    WidgetState::CharacterSelect,
                    CharacterSelection::Random
                )
            )).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_character_select_to_ready_and_sends_event_when_input_attack() {
        let mut controller_input = ControllerInput::default();
        controller_input.attack = true;

        assert!(
            AutexousiousApplication::config_base(
                "updates_widget_character_select_to_ready_and_sends_event_when_input_attack",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| setup_widget(
                world,
                WidgetState::CharacterSelect,
                CharacterSelection::Id(0),
                controller_input
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_widget(
                world,
                CharacterSelectionWidget::new(WidgetState::Ready, CharacterSelection::Id(0))
            )).with_assertion(|world| assert_events(
                world,
                vec![CharacterSelectionEvent::Select {
                    controller_id: 123,
                    character_selection: CharacterSelection::Id(0)
                }]
            )).run()
            .is_ok()
        );
    }

    #[test]
    fn selects_last_character_when_input_left_and_selection_random() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;

        assert!(
            AutexousiousApplication::config_base(
                "selects_previous_character_when_input_left",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| setup_widget(
                world,
                WidgetState::CharacterSelect,
                CharacterSelection::Random,
                controller_input
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_widget(
                world,
                CharacterSelectionWidget::new(
                    WidgetState::CharacterSelect,
                    CharacterSelection::Id(0)
                )
            )).with_assertion(|world| assert_events(
                world,
                vec![CharacterSelectionEvent::Select {
                    controller_id: 123,
                    character_selection: CharacterSelection::Id(0)
                }]
            )).run()
            .is_ok()
        );
    }

    #[test]
    fn updates_widget_ready_to_character_select_and_sends_event_when_input_jump() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;

        assert!(
            AutexousiousApplication::config_base(
                "updates_widget_ready_to_character_select_and_sends_event_when_input_jump",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| setup_widget(
                world,
                WidgetState::Ready,
                CharacterSelection::Id(0),
                controller_input
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_widget(
                world,
                CharacterSelectionWidget::new(
                    WidgetState::CharacterSelect,
                    CharacterSelection::Id(0)
                )
            )).with_assertion(|world| assert_events(
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

        assert!(
            AutexousiousApplication::config_base(
                "updates_widget_character_select_to_inactive_when_input_jump",
                false
            ).with_setup(setup_components)
            .with_setup(setup_event_reader)
            .with_setup(move |world| setup_widget(
                world,
                WidgetState::CharacterSelect,
                CharacterSelection::Id(0),
                controller_input
            )).with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[]
            ).with_assertion(|world| assert_widget(
                world,
                CharacterSelectionWidget::new(WidgetState::Inactive, CharacterSelection::Id(0))
            )).with_assertion(|world| assert_events(world, vec![]))
            .run()
            .is_ok()
        );
    }

    fn setup_components(world: &mut World) {
        CharacterSelectionWidgetUiSystemData::setup(&mut world.res);
    }

    fn setup_event_reader(world: &mut World) {
        let event_channel_reader = world
            .write_resource::<EventChannel<CharacterSelectionEvent>>()
            .register_reader();

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

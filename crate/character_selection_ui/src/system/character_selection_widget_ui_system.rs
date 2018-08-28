use amethyst::ecs::prelude::*;
use character_selection::{CharacterSelections, CharacterSelectionsState};
use game_input::{ControllerId, ControllerInput, InputConfig, InputControlled};

use CharacterSelectionWidget;

/// System that creates and deletes `CharacterSelectionWidget` entities.
///
/// This is not private because consumers may use `CharacterSelectionWidgetUiSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionWidgetUiSystem {
    /// Whether the UI is initialized.
    #[new(value = "false")]
    ui_initialized: bool,
}

type CharacterSelectionWidgetUiSystemData<'s> = (
    Read<'s, CharacterSelections>,
    Read<'s, InputConfig>,
    Entities<'s>,
    WriteStorage<'s, CharacterSelectionWidget>,
    WriteStorage<'s, InputControlled>,
    WriteStorage<'s, ControllerInput>,
);

impl CharacterSelectionWidgetUiSystem {
    fn initialize_ui(
        &mut self,
        input_config: &InputConfig,
        entities: &Entities,
        character_selection_widgets: &mut WriteStorage<CharacterSelectionWidget>,
        input_controlleds: &mut WriteStorage<InputControlled>,
        controller_inputs: &mut WriteStorage<ControllerInput>,
    ) {
        if !self.ui_initialized {
            debug!("Initializing Character Selection UI.");
            self.ui_initialized = true;

            (0..input_config.controller_configs.len()).for_each(|index| {
                let controller_id = index as ControllerId;

                entities
                    .build_entity()
                    .with(
                        CharacterSelectionWidget::default(),
                        character_selection_widgets,
                    ).with(InputControlled::new(controller_id), input_controlleds)
                    .with(ControllerInput::default(), controller_inputs)
                    .build();
            });
        }
    }

    fn terminate_ui(
        &mut self,
        entities: &Entities,
        character_selection_widgets: &mut WriteStorage<CharacterSelectionWidget>,
    ) {
        if self.ui_initialized {
            (&**entities, character_selection_widgets)
                .join()
                .for_each(|(entity, _widget)| {
                    entities
                        .delete(entity)
                        .expect("Failed to delete `CharacterSelectionWidget` entity.")
                });
            self.ui_initialized = false;
        }
    }
}

impl<'s> System<'s> for CharacterSelectionWidgetUiSystem {
    type SystemData = CharacterSelectionWidgetUiSystemData<'s>;

    fn run(
        &mut self,
        (
            character_selections,
            input_config,
            entities,
            mut character_selection_widgets,
            mut input_controlleds,
            mut controller_inputs,
        ): Self::SystemData,
    ) {
        match character_selections.state {
            CharacterSelectionsState::Waiting => self.initialize_ui(
                &input_config,
                &entities,
                &mut character_selection_widgets,
                &mut input_controlleds,
                &mut controller_inputs,
            ),
            CharacterSelectionsState::Confirmed => {
                self.terminate_ui(&entities, &mut character_selection_widgets)
            }
            _ => {}
        };
    }
}

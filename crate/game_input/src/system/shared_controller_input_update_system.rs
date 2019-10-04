use amethyst::ecs::prelude::*;
use derive_new::new;
use typename_derive::TypeName;

use crate::{ControllerInput, InputControlled, SharedInputControlled};

/// Updates the `ControllerInput` component based on input from the sharing controllers.
#[derive(Debug, Default, TypeName, new)]
pub struct SharedControllerInputUpdateSystem;

type SharedControllerInputUpdateSystemData<'s> = (
    ReadStorage<'s, InputControlled>,
    WriteStorage<'s, ControllerInput>,
    ReadStorage<'s, SharedInputControlled>,
    Entities<'s>,
);

impl<'s> System<'s> for SharedControllerInputUpdateSystem {
    type SystemData = SharedControllerInputUpdateSystemData<'s>;

    fn run(
        &mut self,
        (input_controlleds, mut controller_inputs, shared_input_controlleds,  entities): Self::SystemData,
    ) {
        let mut merged_input = (&input_controlleds, &controller_inputs).join().fold(
            ControllerInput::default(),
            |mut merged, (_, controller_input)| {
                merged.x_axis_value += controller_input.x_axis_value;
                merged.z_axis_value += controller_input.z_axis_value;
                merged.defend |= controller_input.defend;
                merged.jump |= controller_input.jump;
                merged.attack |= controller_input.attack;
                merged.special |= controller_input.special;

                merged
            },
        );

        if merged_input.x_axis_value < -1. {
            merged_input.x_axis_value = -1.;
        } else if merged_input.x_axis_value > 1. {
            merged_input.x_axis_value = 1.;
        }

        if merged_input.z_axis_value < -1. {
            merged_input.z_axis_value = -1.;
        } else if merged_input.z_axis_value > 1. {
            merged_input.z_axis_value = 1.;
        }

        for (entity, _) in (&*entities, &shared_input_controlleds).join() {
            controller_inputs
                .insert(entity, merged_input)
                // kcov-ignore-start
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to replace `{}`. Error: `{}`",
                        stringify!(ControllerInput),
                        e
                    )
                });
            // kcov-ignore-end
        }
    }
}

use amethyst::ecs::Entity;
use charge_model::play::ChargeUseEvent;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use input_reaction_model::config::InputReactionRequirement;
use serde::{Deserialize, Serialize};

use crate::{
    config::{CharacterIrrPart, InputReactionRequirementParams},
    play::CharacterIrrSystemData,
};

/// Character input reaction requirement.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterIrr(pub Vec<CharacterIrrPart>);

impl<'s> InputReactionRequirement<'s> for CharacterIrr {
    type SystemData = CharacterIrrSystemData<'s>;

    fn requirement_met(
        &self,
        CharacterIrrSystemData {
            health_pointses,
            skill_pointses,
            charge_tracker_clocks,
            charge_use_modes,
            controller_inputs,
            mirroreds,
            charge_use_ec,
        }: &mut Self::SystemData,
        entity: Entity,
    ) -> bool {
        let (
            health_points,
            skill_points,
            charge_tracker_clock,
            charge_use_mode,
            controller_input,
            mirrored,
        ) = (
            health_pointses.get(entity).copied(),
            skill_pointses.get(entity).copied(),
            charge_tracker_clocks.get(entity).copied(),
            charge_use_modes.get(entity).copied(),
            controller_inputs.get(entity).copied(),
            mirroreds.get(entity).copied(),
        );

        let input_reaction_requirement_params = InputReactionRequirementParams {
            health_points,
            skill_points,
            charge_tracker_clock,
            charge_use_mode,
            controller_input,
            mirrored,
        };

        let met = self.iter().all(|input_reaction_requirement| {
            input_reaction_requirement.is_met(input_reaction_requirement_params)
        });

        if met {
            // Signal charge has been used.
            self.iter()
                .filter_map(|input_reaction_requirement| {
                    if let CharacterIrrPart::Charge(charge_points) = input_reaction_requirement {
                        Some(ChargeUseEvent {
                            entity,
                            charge_points: *charge_points,
                        })
                    } else {
                        None
                    }
                })
                .for_each(|charge_use_event| charge_use_ec.single_write(charge_use_event));
        }

        met
    }
}

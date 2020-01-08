use amethyst::ecs::Entity;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{
    config::{BasicIrrParams, BasicIrrPart, InputReactionRequirement},
    play::BasicIrrSystemData,
};

/// Character input reaction requirement.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct BasicIrr(pub Vec<BasicIrrPart>);

impl<'s> InputReactionRequirement<'s> for BasicIrr {
    type SystemData = BasicIrrSystemData<'s>;

    fn requirement_met(
        &self,
        BasicIrrSystemData {
            controller_inputs,
            mirroreds,
        }: &mut Self::SystemData,
        entity: Entity,
    ) -> bool {
        let (controller_input, mirrored) = (
            controller_inputs.get(entity).copied(),
            mirroreds.get(entity).copied(),
        );

        let basic_irr_params = BasicIrrParams {
            controller_input,
            mirrored,
        };

        self.iter()
            .all(|basic_irr_part| basic_irr_part.is_met(basic_irr_params))
    }
}
